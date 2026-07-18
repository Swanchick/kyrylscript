use std::cell::RefCell;
use std::rc::Rc;

use ks_vm_new::{Constant, Instruction, Program, VMResult};
use ks_vm_new::{KsCall, NativeHelper, NativeRegistry, STRING_TYPE};

use crate::drivers::KsDriver;

struct TestPrint {
    output: Rc<RefCell<String>>,
}

impl KsCall for TestPrint {
    fn call(&mut self, arguments: usize, helper: NativeHelper) -> VMResult<()> {
        let gvs = helper.gvs;

        let mut storage_ids = helper.runner.acc.size_pop(arguments);
        storage_ids.reverse();
        for storage_id in storage_ids {
            let variable = gvs.variable(storage_id)?;
            if variable.value_type != STRING_TYPE {
                gvs.storage_remove_owner(storage_id)?;
                continue;
            }

            let string = gvs.collection_string(variable.value)?;

            let mut output = self.output.borrow_mut();
            output.push_str(string);

            gvs.storage_remove_owner(storage_id)?;
        }

        Ok(())
    }
}

#[test]
fn call_native() -> VMResult<()> {
    let mut native = NativeRegistry::new();
    let output = Rc::new(RefCell::new(String::new()));
    native.functions.push(Box::new(TestPrint {
        output: output.clone(),
    }));

    let instructions = vec![
        Instruction::LoadConst(Constant::String(String::from("Hello, "))),
        Instruction::LoadConst(Constant::String(String::from("world!"))),
        Instruction::CallNative(0, 2),
    ];

    let vm = KsDriver::vm_configured(None, None, Some(native), instructions)?;

    assert_eq!(vm.runners[0].program_counter, 3);

    let output_string = output.borrow().clone();
    assert_eq!(output_string, String::from("Hello, world!"));

    Ok(())
}

#[test]
fn e2e_serialize() -> VMResult<()> {
    let instructions = vec![
        Instruction::Add,
        Instruction::Minus,
        Instruction::Mul,
        Instruction::Div,
        Instruction::Eq,
        Instruction::GreaterEq,
        Instruction::Greater,
        Instruction::LessEq,
        Instruction::Less,
        Instruction::NotEq,
        Instruction::And,
        Instruction::Or,
        Instruction::Not,
        Instruction::Increment,
        Instruction::Decrement,
        Instruction::Clone,
        Instruction::ClearAcc,
        Instruction::Return,
        Instruction::Free(67),
        Instruction::JumpIfFalse(67),
        Instruction::JumpIfTrue(67),
        Instruction::Jump(67),
        Instruction::Store,
        Instruction::Assign,
        Instruction::AssignVariable(67),
        Instruction::AssignCollection,
        Instruction::LoadConst(Constant::Null),
        Instruction::LoadConst(Constant::Boolean(true)),
        Instruction::LoadConst(Constant::Boolean(false)),
        Instruction::LoadConst(Constant::Integer(67)),
        Instruction::LoadConst(Constant::Float(67.67)),
        Instruction::LoadConst(Constant::String(String::from("Six Seven"))),
        Instruction::LoadVar(67),
        Instruction::Call,
        Instruction::CallNative(67, 67),
        Instruction::LoadCapture(67),
        Instruction::LoadFunction(67),
        Instruction::LoadCollection(67),
        Instruction::LoadFromCollection,
        Instruction::CollectionLen,
    ];

    let program = Program::from(instructions.clone());
    let buffer = program.serialize();

    let old_program = Program::from(instructions);
    let new_program = Program::deserialize(buffer)?;

    assert_eq!(new_program, old_program);

    Ok(())
}
