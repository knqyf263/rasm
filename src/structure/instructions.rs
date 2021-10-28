use crate::structure::modules::LocalIdx;

#[derive(Debug, Clone, PartialEq)]
pub struct Expr(pub Vec<Instr>);

#[derive(Debug, Clone, PartialEq)]
pub enum Instr {
    I32Const(i32),
    I64Const(i64),
    F32Const(f32),
    F64Const(f64),
    I32Add,
    I32Sub,
    I32Mul,
    LocalGet(LocalIdx),
    LocalSet(LocalIdx),
}
