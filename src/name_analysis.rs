//! Analysis of how identifier names are used in an letpl program.

use crate::ast;
use crate::ast::nameless::{self, CaptureOffset, StackOffset};
use crate::table::Table;

fn lookup<'a, T: Clone>(bindings: &'a Option<Table<T>>, name: &str) -> Option<&'a T> {
    match bindings {
        Some(bindings) => bindings.lookup(name),
        None => None,
    }
}

struct CaptureTable(Table<nameless::Capture>);

impl CaptureTable {
    fn new() -> Self {
        CaptureTable(Table::new())
    }

    fn add_local_capture(&mut self, name: String, stack_offset: StackOffset) -> CaptureOffset {
        let capture = nameless::Capture::Local(stack_offset);
        self.push(name, capture)
    }

    fn add_capture_capture(
        &mut self,
        name: String,
        outer_capture_offset: CaptureOffset,
    ) -> CaptureOffset {
        let capture = nameless::Capture::Capture(outer_capture_offset);
        self.push(name, capture)
    }

    pub fn lookup(&self, name: &str) -> Option<CaptureOffset> {
        let CaptureTable(table) = self;
        table
            .lookup_offset(name)
            .map(|offset| CaptureOffset(offset))
    }

    pub fn push(&mut self, name: String, capture: nameless::Capture) -> CaptureOffset {
        let CaptureTable(table) = self;
        table.push(name, capture);
        CaptureOffset(table.len() - 1)
    }
}

struct Frame {
    stack_top: StackOffset,
    locals: Option<Table<StackOffset>>,
    captures: CaptureTable,
}

struct StackState {
    stack_top: StackOffset,
    save_stack: Vec<StackOffset>,
    globals: Table<StackOffset>,
    locals: Option<Table<StackOffset>>,
    call_stack: Vec<Frame>,
}

impl StackState {
    fn new() -> Self {
        Self {
            stack_top: StackOffset(0),
            save_stack: Vec::new(),
            globals: Table::new(),
            locals: None,
            call_stack: Vec::new(),
        }
    }

    fn current_bindings(&mut self) -> &mut Table<StackOffset> {
        match self.locals.as_mut() {
            Some(locals) => locals,
            None => &mut self.globals,
        }
    }

    fn push(&mut self) {
        self.stack_top += StackOffset(1);
    }

    fn pop(&mut self) {
        self.stack_top -= StackOffset(1);
    }

    fn save_stack(&mut self) {
        self.save_stack.push(self.stack_top);
    }

    fn restore_stack(&mut self) {
        self.stack_top = self.save_stack.pop().expect("save stack underflow");
    }

    fn begin_scope(&mut self, name: &str) {
        let stack_offset = self.stack_top - StackOffset(1);
        self.current_bindings().push(name.to_string(), stack_offset);
    }

    fn end_scope(&mut self) {
        self.current_bindings().pop();
    }

    fn begin_proc(&mut self, proc_name: &str, param_name: &str) {
        let stack_top = std::mem::replace(&mut self.stack_top, StackOffset(0));
        let locals = std::mem::replace(&mut self.locals, Some(Table::new()));
        let frame = Frame {
            stack_top,
            locals,
            captures: CaptureTable::new(),
        };
        self.call_stack.push(frame);

        // simulate pushing proc object and argument
        self.push();
        self.begin_scope(proc_name);
        self.push();
        self.begin_scope(param_name);
    }

    fn end_proc(&mut self) -> CaptureTable {
        let frame = self.call_stack.pop().expect("call stack underflow");
        self.stack_top = frame.stack_top;
        self.locals = frame.locals;
        frame.captures
    }

    fn lookup_local(&mut self, name: &str) -> Option<&StackOffset> {
        lookup(&self.locals, name)
    }

    fn lookup_capture(&mut self, name: &str) -> Option<CaptureOffset> {
        let call_depth = self.call_stack.len();
        if call_depth > 0 {
            self.capture(name, call_depth - 1)
        } else {
            None
        }
    }

    fn capture(&mut self, name: &str, call_depth: usize) -> Option<CaptureOffset> {
        let frame = &mut self.call_stack[call_depth];
        if let Some(stack_offset) = lookup(&frame.locals, name) {
            let capture_offset = frame
                .captures
                .add_local_capture(name.to_string(), *stack_offset);
            Some(capture_offset)
        } else if let Some(capture_offset) = frame.captures.lookup(name) {
            Some(capture_offset)
        } else if call_depth > 0 {
            self.capture(name, call_depth - 1)
                .map(|outer_capture_offset| {
                    self.call_stack[call_depth]
                        .captures
                        .add_capture_capture(name.to_string(), outer_capture_offset)
                })
        } else {
            None
        }
    }
}

pub fn resolve_names(program: &ast::Program) -> Result<nameless::Program, String> {
    let mut state = StackState::new();
    let expr = resolve_names_expr(&program.expr, &mut state)?;
    Ok(nameless::Program { expr })
}

fn resolve_names_expr(
    expr: &ast::Expr,
    state: &mut StackState,
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
            state.push();
            if let Some(&stack_offset) = state.lookup_local(name) {
                Ok(Box::new(nameless::Expr::Local(stack_offset)))
            } else if let Some(capture_offset) = state.lookup_capture(name) {
                Ok(Box::new(nameless::Expr::Capture(capture_offset)))
            } else if let Some(&stack_offset) = state.globals.lookup(name) {
                Ok(Box::new(nameless::Expr::Global(stack_offset)))
            } else {
                Err(format!("undefined name: {name}"))
            }
        }
    }
}

fn resolve_names_proc(
    proc_name: &str,
    param_name: &str,
    body: &ast::Expr,
    state: &mut StackState,
) -> Result<Box<nameless::Expr>, String> {
    state.begin_proc(proc_name, param_name);
    let body = resolve_names_expr(body, state)?;
    let CaptureTable(capture_table) = state.end_proc();
    let captures: Vec<nameless::Capture> =
        capture_table.items.iter().map(|item| item.value).collect();
    state.push();
    Ok(Box::new(nameless::Expr::Proc { body, captures }))
}
