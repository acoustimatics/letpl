use std::fmt;

use crate::parser;

pub struct Program {
    pub expr: Box<Expr>,
}

pub enum Expr {
    Call(Box<Expr>, Box<Expr>),

    Capture(usize),

    Const(f64),

    Diff(Box<Expr>, Box<Expr>),

    Global(usize),

    IsZero(Box<Expr>),

    If(Box<Expr>, Box<Expr>, Box<Expr>),

    Let(Box<Expr>, Box<Expr>),

    Local(usize),

    Print(Box<Expr>),

    Proc(Box<Expr>, Vec<Cap>),
}

pub enum Cap {
    Local(usize),
    Capture(usize),
}

#[derive(Clone)]
struct Binding {
    name: String,
    stack_index: usize,
}

impl Binding {
    fn new(name: &str, stack_index: usize) -> Self {
        let name = name.to_owned();
        Self { name, stack_index }
    }
}

#[derive(Clone)]
struct BindingTable {
    bindings: Vec<Binding>,
}

impl BindingTable {
    fn new() -> Self {
        let bindings = Vec::new();
        Self { bindings }
    }

    fn push(&mut self, name: &str, stack_index: usize) {
        let binding = Binding::new(name, stack_index);
        self.bindings.push(binding);
    }

    fn pop(&mut self) {
        self.bindings.pop().expect("binding table underflow");
    }

    fn lookup(&self, lookup_name: &str) -> Option<usize> {
        self.bindings
            .iter()
            .rev()
            .find(|b| b.name == lookup_name)
            .map(|b| b.stack_index)
    }
}

impl fmt::Debug for BindingTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ ")?;
        for binding in self.bindings.iter() {
            write!(f, "{}#{} ", binding.name, binding.stack_index)?;
        }
        write!(f, "}}")
    }
}

fn lookup(bindings: &Option<BindingTable>, lookup_name: &str) -> Option<usize> {
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

impl fmt::Debug for CaptureTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ ")?;
        for c in self.captures.iter() {
            let kind = if c.is_local { "local" } else { "capture" };
            write!(f, "{}:{}#{} ", c.name, kind, c.index)?;
        }
        write!(f, "}}")
    }
}

struct Frame {
    stack_top: usize,
    locals: Option<BindingTable>,
    captures: CaptureTable,
}

struct CompilerState {
    stack_top: usize,
    save_stack: Vec<usize>,
    globals: BindingTable,
    locals: Option<BindingTable>,
    call_stack: Vec<Frame>,
}

impl CompilerState {
    fn new() -> Self {
        Self {
            stack_top: 0,
            save_stack: Vec::new(),
            globals: BindingTable::new(),
            locals: None,
            call_stack: Vec::new(),
        }
    }

    fn current_bindings(&mut self) -> &mut BindingTable {
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
        self.current_bindings().push(name, stack_index);
    }

    fn end_scope(&mut self) {
        self.current_bindings().pop()
    }

    fn begin_proc(&mut self, name: &str, var: &str) {
        let stack_top = std::mem::replace(&mut self.stack_top, 0);
        let locals = std::mem::replace(&mut self.locals, Some(BindingTable::new()));
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

    fn lookup_local(&mut self, lookup_name: &str) -> Option<usize> {
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
            let capture_index = frame.captures.add_local_capture(lookup_name, stack_index);
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

pub fn resolve_names(program: &parser::Program) -> Result<Program, String> {
    let mut state = CompilerState::new();
    let expr = resolve_names_expr(&program.expr, &mut state)?;
    Ok(Program { expr })
}

fn resolve_names_expr(expr: &parser::Expr, state: &mut CompilerState) -> Result<Box<Expr>, String> {
    match expr {
        parser::Expr::Call(proc, arg) => {
            let proc = resolve_names_expr(proc, state)?;
            let arg = resolve_names_expr(arg, state)?;
            state.pop();
            state.pop();
            state.push();
            Ok(Box::new(Expr::Call(proc, arg)))
        }

        parser::Expr::Const(x) => {
            state.push();
            Ok(Box::new(Expr::Const(*x)))
        }

        parser::Expr::Diff(lhs, rhs) => {
            let lhs = resolve_names_expr(lhs, state)?;
            let rhs = resolve_names_expr(rhs, state)?;
            state.pop();
            state.pop();
            state.push();
            Ok(Box::new(Expr::Diff(lhs, rhs)))
        }

        parser::Expr::If(guard, consq, alt) => {
            let guard = resolve_names_expr(guard, state)?;
            state.pop();
            state.save_stack();
            let alt = resolve_names_expr(alt, state)?;
            state.restore_stack();
            let consq = resolve_names_expr(consq, state)?;
            Ok(Box::new(Expr::If(guard, consq, alt)))
        }

        parser::Expr::IsZero(e) => {
            let e = resolve_names_expr(e, state)?;
            state.pop();
            state.push();
            Ok(Box::new(Expr::IsZero(e)))
        }

        parser::Expr::Let(var, rhs, body) => {
            let rhs = resolve_names_expr(rhs, state)?;
            state.begin_scope(var);
            let body = resolve_names_expr(body, state)?;
            state.end_scope();
            Ok(Box::new(Expr::Let(rhs, body)))
        }

        parser::Expr::LetRec {
            name,
            var,
            proc_body,
            let_body,
            ..
        } => {
            let proc = resolve_names_proc(name, var, proc_body, state)?;
            state.begin_scope(name);
            let let_body = resolve_names_expr(let_body, state)?;
            state.end_scope();
            Ok(Box::new(Expr::Let(proc, let_body)))
        }

        parser::Expr::Print(e) => {
            let e = resolve_names_expr(e, state)?;
            Ok(Box::new(Expr::Print(e)))
        }

        parser::Expr::Proc(var, _, body) => resolve_names_proc("", var, body, state),

        parser::Expr::Var(var) => {
            if let Some(stack_index) = state.lookup_local(var) {
                state.push();
                Ok(Box::new(Expr::Local(stack_index)))
            } else if let Some(capture_index) = state.lookup_capture(var) {
                state.push();
                Ok(Box::new(Expr::Capture(capture_index)))
            } else if let Some(stack_index) = state.globals.lookup(var) {
                state.push();
                Ok(Box::new(Expr::Global(stack_index)))
            } else {
                Err(format!("undefined name: {}", var))
            }
        }
    }
}

fn resolve_names_proc(
    name: &str,
    var: &str,
    body: &Box<parser::Expr>,
    state: &mut CompilerState,
) -> Result<Box<Expr>, String> {
    state.begin_proc(name, var);
    let body = resolve_names_expr(body, state)?;
    let captures = state.end_proc();
    let captures: Vec<Cap> = captures
        .captures
        .iter()
        .map(|c| {
            if c.is_local {
                Cap::Local(c.index)
            } else {
                Cap::Capture(c.index)
            }
        })
        .collect();
    state.push();
    Ok(Box::new(Expr::Proc(body, captures)))
}
