use crate::errors::RuntimeError;
use crate::execution::runtime::{ModuleInst, Store};
use crate::execution::stack::{Frame, FrameStack, LabelStack, Val, ValueStack};
use crate::structure::modules::ExportDesc::Func;
use crate::structure::modules::Module;
use std::error::Error;

impl Module {
    pub fn call(
        &self,
        store: Store,
        name: &String,
        args: Vec<Val>,
    ) -> Result<Vec<Val>, Box<dyn Error>> {
        let export = self
            .exports
            .iter()
            .find(|&e| e.name.0 == *name)
            .ok_or("unknown function")?;
        let funcaddr = if let Func(x) = &export.desc {
            x
        } else {
            return Err(Box::new(RuntimeError::InvalidParameters(
                "unknown function".to_string(),
            )));
        };

        let funcinst = store
            .funcs
            .get((*funcaddr).0 as usize)
            .ok_or("unknown func")?;
        if args.len() != (*funcinst).type_.0 .0.len() {
            return Err(Box::new(RuntimeError::InvalidParameters(
                "invalid args".to_string(),
            )));
        }

        let mut frame_stack = FrameStack(vec![]);
        let mut value_stack = ValueStack(vec![]);

        // TODO: Push the dummy frame
        frame_stack.push(Frame {
            locals: &mut vec![],
            module: &mut Default::default(),
            labels: LabelStack(vec![]),
        });

        // Push the args to the stack
        for v in args {
            value_stack.push(v);
        }

        funcinst.call(&mut value_stack, &mut frame_stack)?;

        let mut results = vec![];
        for _ in &funcinst.type_.1 .0 {
            let t = value_stack.pop().ok_or("result error")?;
            results.push(t);
        }

        Ok(results)
    }
}
