//! Analysis of how identifier names are used in an letpl program.

use crate::ast;
use crate::ast::nameless;
use crate::symbol_table::SymbolTable;

fn lookup<'a>(bindings: &'a Option<SymbolTable<usize>>, lookup_name: &str) -> Option<&'a usize> {
    match bindings {
        Some(bindings) => bindings.lookup(lookup_name),
        None => None,
    }
}

#[derive(Debug)]
struct Capture {
    name: String,
    is_local: bool,
    index: usize,
}

struct CaptureTable {
    captures: Vec<Capture>,
}

impl CaptureTable {
    fn new() -> Self {
        Self {
            captures: Vec::new(),
        }
    }

    fn add_local_capture(&mut self, name: &str, scope: usize) -> usize {
        let name = name.to_owned();
        let is_local = true;
        let index = scope;
        let capture = Capture {
            name,
            is_local,
            index,
        };
        self.captures.push(capture);
        self.captures.len() - 1
    }

    fn add_capture_capture(&mut self, name: &str, outer_index: usize) -> usize {
        let name = name.to_owned();
        let is_local = false;
        let index = outer_index;
        let capture = Capture {
            name,
            is_local,
            index,
        };
        self.captures.push(capture);
        self.captures.len() - 1
    }

    pub fn lookup(&self, lookup_name: &str) -> Option<usize> {
        self.captures
            .iter()
            .enumerate()
            .rev()
            .find(|(_, c)| c.name == lookup_name)
            .map(|(i, _)| i)
    }
}

struct Frame {
    stack_top: usize,
    locals: Option<SymbolTable<usize>>,
    captures: CaptureTable,
}

struct CompilerState {
    stack_top: usize,
    save_stack: Vec<usize>,
    globals: SymbolTable<usize>,
    locals: Option<SymbolTable<usize>>,
    call_stack: Vec<Frame>,
}

impl CompilerState {
    fn new() -> Self {
        Self {
            stack_top: 0,
            save_stack: Vec::new(),
            globals: SymbolTable::new(),
            locals: None,
            call_stack: Vec::new(),
        }
    }

    fn current_bindings(&mut self) -> &mut SymbolTable<usize> {
        match self.locals.as_mut() {
            Some(locals) => locals,
            None => &mut self.globals,
        }
    }

    fn push(&mut self) {
        self.stack_top += 1;
    }

    fn pop(&mut self) {
        self.stack_top -= 1;
    }

    fn save_stack(&mut self) {
        self.save_stack.push(self.stack_top);
    }

    fn restore_stack(&mut self) {
        self.stack_top = self.save_stack.pop().expect("save stack underflow");
    }

    fn begin_scope(&mut self, name: &str) {
        let stack_index = self.stack_top - 1;
        self.current_bindings().push(name, &stack_index);
    }

    fn end_scope(&mut self) {
        self.current_bindings().pop();
    }

    fn begin_proc(&mut self, name: &str, var: &str) {
        let stack_top = std::mem::replace(&mut self.stack_top, 0);
        let locals = std::mem::replace(&mut self.locals, Some(SymbolTable::new()));
        let frame = Frame {
            stack_top,
            locals,
            captures: CaptureTable::new(),
        };
        self.call_stack.push(frame);

        // simulate pushing proc object and argument
        self.push();
        self.begin_scope(name);
        self.push();
        self.begin_scope(var);
    }

    fn end_proc(&mut self) -> CaptureTable {
        let frame = self.call_stack.pop().expect("call stack underflow");
        self.stack_top = frame.stack_top;
        self.locals = frame.locals;
        frame.captures
    }

    fn lookup_local(&mut self, lookup_name: &str) -> Option<&usize> {
        lookup(&self.locals, lookup_name)
    }

    fn lookup_capture(&mut self, lookup_name: &str) -> Option<usize> {
        let call_depth = self.call_stack.len();
        if call_depth > 0 {
            self.capture(lookup_name, call_depth - 1)
        } else {
            None
        }
    }

    fn capture(&mut self, lookup_name: &str, call_depth: usize) -> Option<usize> {
        let frame = &mut self.call_stack[call_depth];
        if let Some(stack_index) = lookup(&frame.locals, lookup_name) {
            let capture_index = frame.captures.add_local_capture(lookup_name, *stack_index);
            Some(capture_index)
        } else if let Some(capture_index) = frame.captures.lookup(lookup_name) {
            Some(capture_index)
        } else if call_depth > 0 {
            self.capture(lookup_name, call_depth - 1)
                .map(|outer_capture_index| {
                    self.call_stack[call_depth]
                        .captures
                        .add_capture_capture(lookup_name, outer_capture_index)
                })
        } else {
            None
        }
    }
}

pub fn resolve_names(program: &ast::Program) -> Result<nameless::Program, String> {
    let mut state = CompilerState::new();
    let expr = resolve_names_expr(&program.expr, &mut state)?;
    Ok(nameless::Program { expr })
}

fn resolve_names_expr(
    expr: &ast::Expr,
    state: &mut CompilerState,
) -> Result<Box<nameless::Expr>, String> {
    match expr {
        ast::Expr::Assert { line, test, body } => {
            let test = resolve_names_expr(test, state)?;
            state.pop();
            let body = resolve_names_expr(body, state)?;
            Ok(Box::new(nameless::Expr::Assert {
                line: *line,
                test,
                body,
            }))
        }

        ast::Expr::Call { proc, arg } => {
            let proc = resolve_names_expr(proc, state)?;
            let arg = resolve_names_expr(arg, state)?;
            state.pop();
            state.pop();
            state.push();
            Ok(Box::new(nameless::Expr::Call { proc, arg }))
        }

        ast::Expr::LiteralInt(x) => {
            state.push();
            Ok(Box::new(nameless::Expr::LiteralInt(*x)))
        }

        ast::Expr::Subtract { left, right } => {
            let left = resolve_names_expr(left, state)?;
            let right = resolve_names_expr(right, state)?;
            state.pop();
            state.pop();
            state.push();
            Ok(Box::new(nameless::Expr::Subtract { left, right }))
        }

        ast::Expr::If {
            test,
            consequent,
            alternate,
        } => {
            let test = resolve_names_expr(test, state)?;
            state.pop();
            state.save_stack();
            let alternate = resolve_names_expr(alternate, state)?;
            state.restore_stack();
            let consequent = resolve_names_expr(consequent, state)?;
            Ok(Box::new(nameless::Expr::If {
                test,
                consequent,
                alternate,
            }))
        }

        ast::Expr::IsZero(e) => {
            let e = resolve_names_expr(e, state)?;
            state.pop();
            state.push();
            Ok(Box::new(nameless::Expr::IsZero(e)))
        }

        ast::Expr::Let { name, expr, body } => {
            let expr = resolve_names_expr(expr, state)?;
            state.begin_scope(name);
            let body = resolve_names_expr(body, state)?;
            state.end_scope();
            Ok(Box::new(nameless::Expr::Let { expr, body }))
        }

        ast::Expr::LetRec {
            name,
            param,
            proc_body,
            let_body,
            ..
        } => {
            let expr = resolve_names_proc(name, &param.name, proc_body, state)?;
            state.begin_scope(name);
            let body = resolve_names_expr(let_body, state)?;
            state.end_scope();
            Ok(Box::new(nameless::Expr::Let { expr, body }))
        }

        ast::Expr::LiteralBool(value) => {
            state.push();
            Ok(Box::new(nameless::Expr::LiteralBool(*value)))
        }

        ast::Expr::Proc { param, body } => resolve_names_proc("", &param.name, body, state),

        ast::Expr::Name(name) => {
            if let Some(&stack_index) = state.lookup_local(name) {
                state.push();
                Ok(Box::new(nameless::Expr::Local(stack_index)))
            } else if let Some(capture_index) = state.lookup_capture(name) {
                state.push();
                Ok(Box::new(nameless::Expr::Capture(capture_index)))
            } else if let Some(&stack_index) = state.globals.lookup(name) {
                state.push();
                Ok(Box::new(nameless::Expr::Global(stack_index)))
            } else {
                Err(format!("undefined name: {name}"))
            }
        }
    }
}

fn resolve_names_proc(
    name: &str,
    var: &str,
    body: &ast::Expr,
    state: &mut CompilerState,
) -> Result<Box<nameless::Expr>, String> {
    state.begin_proc(name, var);
    let body = resolve_names_expr(body, state)?;
    let captures = state.end_proc();
    let captures: Vec<nameless::Capture> = captures
        .captures
        .iter()
        .map(|c| {
            if c.is_local {
                nameless::Capture::Local(c.index)
            } else {
                nameless::Capture::Capture(c.index)
            }
        })
        .collect();
    state.push();
    Ok(Box::new(nameless::Expr::Proc { body, captures }))
}
