use crate::structure::modules::{Func, Global, Module};
use crate::structure::types::{FuncType, TableType};
use std::borrow::BorrowMut;
use std::error::Error;
use std::rc::Weak;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Store {
    pub funcs: Vec<FuncInst>,
    pub tables: Vec<TableInst>,
    pub mems: Vec<MemInst>,
    pub globals: Vec<GlobalInst>,
    pub elems: Vec<ElemInst>,
    pub datas: Vec<DataInst>,
}

impl Store {
    pub fn new(module: &Module) -> Result<Store, Box<dyn Error>> {
        let mut store: Store = Default::default();
        let mut moduleinst: ModuleInst = Default::default();
        for (i, f) in module.funcs.iter().enumerate() {
            let functype = module
                .types
                .get(f.type_.0 as usize)
                .ok_or("unknown functype")?;
            moduleinst.funcaddrs.push(i);
            store.funcs.push(FuncInst {
                type_: functype.clone(),
                module: &mut moduleinst,
                code: f.clone(),
            });
        }
        Ok(store)
    }
}

type Addr = usize;

// Module Instances: https://webassembly.github.io/spec/core/exec/runtime.html#module-instances
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ModuleInst {
    pub types: Vec<FuncType>,
    pub funcaddrs: Vec<Addr>,
    pub tableaddrs: Vec<Addr>,
    pub memaddrs: Vec<Addr>,
    pub globaladdrs: Vec<Addr>,
    pub elemaddrs: Vec<Addr>,
    pub dataaddrs: Vec<Addr>,
    pub exports: Vec<Addr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncInst {
    pub type_: FuncType,
    pub module: *mut ModuleInst,
    pub code: Func,
}

// TODO
#[derive(Debug, Clone, PartialEq)]
pub struct TableInst {}

// TODO
#[derive(Debug, Clone, PartialEq)]
pub struct MemInst {}

// TODO
#[derive(Debug, Clone, PartialEq)]
pub struct GlobalInst {}

// TODO
#[derive(Debug, Clone, PartialEq)]
pub struct ElemInst {}

// TODO
#[derive(Debug, Clone, PartialEq)]
pub struct DataInst {}
