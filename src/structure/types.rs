#[derive(Debug, Clone, PartialEq)]
pub enum NumType {
    I32,
    I64,
    F32,
    F64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RefType {
    FuncRef,
    ExternRef,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValType {
    NumType(NumType),
    RefType(RefType),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResultType(pub Vec<ValType>);

#[derive(Debug, Clone, PartialEq)]
pub struct FuncType(pub ResultType, pub ResultType);

#[derive(Debug, Clone, PartialEq)]
pub struct Limits {
    pub min: u32,
    pub max: Option<u32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemType(pub Limits);

#[derive(Debug, Clone, PartialEq)]
pub struct TableType(pub Limits, pub RefType);

// Global Types: https://webassembly.github.io/spec/core/syntax/types.html#global-types
#[derive(Debug, Clone, PartialEq)]
pub struct GlobalType(pub Mut, pub ValType);

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Mut {
    Const,
    Var,
}
