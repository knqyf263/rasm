use super::instructions::Expr;
use super::types::{FuncType, GlobalType, MemType, TableType, ValType};
use crate::structure::types::RefType;
use crate::structure::values::{Byte, Name};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Module {
    pub types: Vec<FuncType>,
    pub funcs: Vec<Func>,
    pub tables: Vec<Table>,
    pub mems: Vec<Mem>,
    pub globals: Vec<Global>,
    pub elems: Vec<Elem>,
    pub datas: Vec<Data>,
    pub start: Option<Start>,
    pub imports: Vec<Import>,
    pub exports: Vec<Export>,
}

// Indices: https://webassembly.github.io/spec/core/syntax/modules.html#indices
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct TypeIdx(pub u32);

#[derive(Debug, Clone, PartialEq)]
pub struct FuncIdx(pub u32);

#[derive(Debug, Clone, PartialEq)]
pub struct TableIdx(pub u32);

#[derive(Debug, Clone, PartialEq)]
pub struct MemIdx(pub u32);

#[derive(Debug, Clone, PartialEq)]
pub struct GlobalIdx(pub u32);

#[derive(Debug, Clone, PartialEq)]
pub struct ElemIdx(pub u32);

#[derive(Debug, Clone, PartialEq)]
pub struct DataIdx(pub u32);

#[derive(Debug, Clone, PartialEq)]
pub struct LocalIdx(pub u32);

#[derive(Debug, Clone, PartialEq)]
pub struct LabelIdx(pub u32);

// Func: https://webassembly.github.io/spec/core/syntax/modules.html#functions
#[derive(Debug, Clone, PartialEq)]
pub struct Func {
    pub type_: TypeIdx,
    pub locals: Vec<ValType>,
    pub body: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Table {
    pub type_: TableType,
}

// Memories: https://webassembly.github.io/spec/core/syntax/modules.html#syntax-mem
#[derive(Debug, Clone, PartialEq)]
pub struct Mem {
    pub type_: MemType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Global {
    pub type_: GlobalType,
    pub init: Expr,
}

// Element Segments: https://webassembly.github.io/spec/core/syntax/modules.html#element-segments
#[derive(Debug, Clone, PartialEq)]
pub struct Elem {
    pub type_: RefType,
    pub init: Vec<Expr>,
    pub mode: ElemMode,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ElemMode {
    Passive,
    Active,
    Declarative,
}

// Data Segments
#[derive(Debug, Clone, PartialEq)]
pub struct Data {
    pub init: Vec<Byte>,
    pub mode: DataMode,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataMode {
    Passive,
    Active,
}

// Start Function: https://webassembly.github.io/spec/core/syntax/modules.html#start-function
#[derive(Debug, Clone, PartialEq)]
pub struct Start {
    pub func: FuncIdx,
}

// Exports: https://webassembly.github.io/spec/core/syntax/modules.html#exports
#[derive(Debug, Clone, PartialEq)]
pub struct Export {
    pub name: Name,
    pub desc: ExportDesc,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExportDesc {
    Func(FuncIdx),
    Table(TableIdx),
    Mem(MemIdx),
    Global(GlobalIdx),
}

// Imports: https://webassembly.github.io/spec/core/syntax/modules.html#imports
#[derive(Debug, Clone, PartialEq)]
pub struct Import {
    pub module: Name,
    pub name: Name,
    pub desc: ImportDesc,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImportDesc {
    Func(FuncIdx),
    Table(TableIdx),
    Mem(MemIdx),
    Global(GlobalIdx),
}
