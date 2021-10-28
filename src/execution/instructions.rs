use crate::errors::RuntimeError::InvalidWasmError;
use crate::execution::runtime::FuncInst;
use crate::execution::stack::Val::{F32, F64, I32, I64};
use crate::execution::stack::{Frame, FrameStack, LabelStack, Val, ValueStack};
use crate::structure::instructions::Instr;
use crate::structure::types::ValType;
use std::error::Error;

impl FuncInst {
    pub fn call(
        &self,
        value_stack: &mut ValueStack,
        frame_stack: &mut FrameStack,
    ) -> Result<(), Box<dyn Error>> {
        let mut locals = vec![];

        for _ in &self.type_.0 .0 {
            let v = value_stack.pop().ok_or("result error")?;
            locals.push(v);
        }
        locals.reverse();

        for _ in &self.code.locals {
            locals.push(Val::I32(0));
        }

        frame_stack.push(Frame {
            locals: &mut locals,
            module: self.module,
            labels: LabelStack(vec![]),
        });

        for instr in &self.code.body.0 {
            self.invoke(&instr, &mut locals, value_stack)?;
        }

        Ok(())
    }

    fn invoke(
        &self,
        instr: &Instr,
        locals: &mut Vec<Val>,
        stack: &mut ValueStack,
    ) -> Result<(), Box<dyn Error>> {
        match instr {
            Instr::I32Const(num) => stack.push(I32(*num)),
            Instr::I64Const(num) => stack.push(I64(*num)),
            Instr::F32Const(num) => stack.push(F32(*num)),
            Instr::F64Const(num) => stack.push(F64(*num)),
            Instr::I32Add => {
                let mut a2 = i32_pop(stack)?;
                let mut a1 = i32_pop(stack)?;
                stack.push(I32(a1 + a2));
            }
            Instr::I32Sub => {
                let mut a2 = i32_pop(stack)?;
                let mut a1 = i32_pop(stack)?;
                stack.push(I32(a1 - a2));
            }
            Instr::I32Mul => {
                let mut a2 = i32_pop(stack)?;
                let mut a1 = i32_pop(stack)?;
                stack.push(I32(a1 * a2));
            }
            Instr::LocalGet(idx) => {
                let idx = (*idx).0 as usize;
                let v = locals.get(idx).ok_or("invalid locals")?;
                stack.push(*v)
            }
            Instr::LocalSet(idx) => {
                let idx = (*idx).0 as usize;
                let v = stack.pop().ok_or("empty stack")?;
                locals[idx] = v;
            }
            _ => {
                return Err(Box::new(InvalidWasmError(
                    "not implemented instruction".to_string(),
                )));
            }
        }
        Ok(())
    }
}

fn i32_pop(stack: &mut ValueStack) -> Result<i32, Box<dyn Error>> {
    let num = if let I32(num) = stack.pop().ok_or("no param")? {
        num
    } else {
        return Err(Box::new(InvalidWasmError("invalid param type".to_string())));
    };

    Ok(num)
}
