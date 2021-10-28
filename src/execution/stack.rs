use crate::execution::runtime::ModuleInst;
use crate::structure::instructions::Instr;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Val {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ValueStack(pub Vec<Val>);

impl ValueStack {
    pub fn push(&mut self, v: Val) {
        self.0.push(v);
    }

    pub fn pop(&mut self) -> Option<Val> {
        self.0.pop()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Label {
    pub instrs: Vec<Instr>,
    pub n: usize,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct LabelStack(pub Vec<Label>);

#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    pub locals: *mut Vec<Val>,
    pub module: *mut ModuleInst,
    pub labels: LabelStack,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FrameStack(pub Vec<Frame>);

impl FrameStack {
    pub fn push(&mut self, f: Frame) {
        self.0.push(f);
    }

    pub fn pop(&mut self) -> Option<Frame> {
        self.0.pop()
    }
}
