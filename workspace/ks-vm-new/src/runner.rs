// #[cfg(not(feature = "std"))]
use alloc::vec::Vec;
// #[cfg(not(feature = "std"))]
use alloc::format;
// #[cfg(not(feature = "std"))]
use alloc::string::{String, ToString};

use crate::types::{Arguments, NativeId};
use crate::{Assign, Function, NativeCall, VMError, VMResult};

use super::call_stack::CallStack;
use super::environment::variable::{
    BOOLEAN_TYPE, FLOAT_TYPE, INT_TYPE, NULL_TYPE, STACK_TYPE, STRING_TYPE,
};
use super::environment::{GVS, Stack, Variable};
use super::types::{CaptureSize, CollectionId, Offset, Pointer, Slot, StorageId};
use super::{Constant, Instruction};

#[derive(Debug)]
pub struct Runner {
    pub program_counter: Pointer,
    pub acc: Stack,
    pub stack: Stack,
    pub call_stack: Vec<CallStack>,
    pub assign: Assign,
    pub prevent_step: bool,
}

impl Default for Runner {
    fn default() -> Self {
        Self::new()
    }
}

impl Runner {
    pub fn new() -> Self {
        Self {
            program_counter: 0,
            acc: Stack::new(),
            stack: Stack::new(),
            call_stack: Vec::new(),
            assign: Assign::None,
            prevent_step: false,
        }
    }

    fn step(&mut self) {
        if !self.prevent_step {
            self.program_counter += 1;
        }

        self.prevent_step = false;
    }

    fn load_const_string(gvs: &mut GVS, string: String) -> Variable {
        let collection_id = gvs.collection_store_string(string);

        Variable::string(collection_id)
    }

    fn load_const(&mut self, gvs: &mut GVS, constant: Constant) -> VMResult<()> {
        let variable = match constant {
            Constant::Null => Variable::null(),
            Constant::Integer(value) => Variable::from(value),
            Constant::Float(value) => Variable::from(value),
            Constant::Boolean(value) => Variable::from(value),
            Constant::String(string) => Self::load_const_string(gvs, string),
        };

        self.acc.push(gvs, variable)?;

        Ok(())
    }

    fn load_var(&mut self, gvs: &mut GVS, slot: Slot) -> VMResult<()> {
        let storage_id = self.stack.storage_id(slot)?;
        self.acc.push_storage_id(gvs, storage_id)?;

        Ok(())
    }

    fn jump(&mut self, offset: Offset) -> VMResult<()> {
        self.program_counter = if offset < 0 {
            self.program_counter
                .saturating_sub(offset.unsigned_abs() as Pointer)
        } else {
            self.program_counter.saturating_add(offset as Pointer)
        };

        self.prevent_step = true;

        Ok(())
    }

    fn add_strings(gvs: &mut GVS, left: CollectionId, right: CollectionId) -> VMResult<u64> {
        let mut left = gvs.collection_string(left)?.to_string();
        let right = gvs.collection_string(right)?;

        left.push_str(right);
        let collection_id = gvs.collection_store_string(left);

        Ok(collection_id)
    }

    fn add(&mut self, gvs: &mut GVS) -> VMResult<()> {
        let right = self.acc.pop(gvs)?;
        let left = self.acc.pop(gvs)?;

        let variable = match (left.value_type, right.value_type) {
            (INT_TYPE, INT_TYPE) => Ok(Variable::from(left.value as i64 + right.value as i64)),
            (INT_TYPE, FLOAT_TYPE) | (FLOAT_TYPE, INT_TYPE) | (FLOAT_TYPE, FLOAT_TYPE) => {
                Ok(Variable::from(left.as_f64()? + right.as_f64()?))
            }
            (STRING_TYPE, STRING_TYPE) => {
                let collection_id = Self::add_strings(gvs, left.value, right.value)?;
                Ok(Variable::string(collection_id))
            }
            _ => Err("Invalid type"),
        }?;

        self.acc.push(gvs, variable)?;

        Ok(())
    }

    fn binary_op<RI, RF>(
        &mut self,
        gvs: &mut GVS,
        operation_int: impl Fn(i64, i64) -> RI,
        operation_float: impl Fn(f64, f64) -> RF,
    ) -> VMResult<()>
    where
        Variable: From<RI> + From<RF>,
    {
        let right = self.acc.pop(gvs)?;
        let left = self.acc.pop(gvs)?;

        let variable = match (left.value_type, right.value_type) {
            (INT_TYPE, INT_TYPE) => Ok(Variable::from(operation_int(
                left.value as i64,
                right.value as i64,
            ))),
            (INT_TYPE, FLOAT_TYPE) | (FLOAT_TYPE, INT_TYPE) | (FLOAT_TYPE, FLOAT_TYPE) => Ok(
                Variable::from(operation_float(left.as_f64()?, right.as_f64()?)),
            ),
            _ => Err("Invalid type"),
        }?;

        self.acc.push(gvs, variable)?;

        Ok(())
    }

    fn minus(&mut self, gvs: &mut GVS) -> VMResult<()> {
        self.binary_op(gvs, |l, r| l - r, |l, r| l - r)?;
        Ok(())
    }

    fn mul(&mut self, gvs: &mut GVS) -> VMResult<()> {
        self.binary_op(gvs, |l, r| l * r, |l, r| l * r)?;
        Ok(())
    }

    fn check_zero_division(&self, gvs: &mut GVS) -> VMResult<()> {
        let variable = self.acc.last(gvs)?;
        let float_value = variable.as_f64()?;

        if (variable.value_type == INT_TYPE || variable.value_type == FLOAT_TYPE)
            && float_value == 0.0
        {
            return Err(VMError::from("Zero division error"));
        }

        Ok(())
    }

    fn div(&mut self, gvs: &mut GVS) -> VMResult<()> {
        self.check_zero_division(gvs)?;
        let right = self.acc.pop(gvs)?;
        let left = self.acc.pop(gvs)?;

        let variable = match (left.value_type, right.value_type) {
            (INT_TYPE, INT_TYPE)
            | (INT_TYPE, FLOAT_TYPE)
            | (FLOAT_TYPE, INT_TYPE)
            | (FLOAT_TYPE, FLOAT_TYPE) => Ok(Variable::from(left.as_f64()? / right.as_f64()?)),
            _ => Err("Invalid type"),
        }?;

        self.acc.push(gvs, variable)?;

        Ok(())
    }

    fn greater_eq(&mut self, gvs: &mut GVS) -> VMResult<()> {
        self.binary_op(gvs, |l, r| l >= r, |l, r| l >= r)?;
        Ok(())
    }

    fn greater(&mut self, gvs: &mut GVS) -> VMResult<()> {
        self.binary_op(gvs, |l, r| l > r, |l, r| l > r)?;
        Ok(())
    }

    fn less_eq(&mut self, gvs: &mut GVS) -> VMResult<()> {
        self.binary_op(gvs, |l, r| l <= r, |l, r| l <= r)?;
        Ok(())
    }

    fn less(&mut self, gvs: &mut GVS) -> VMResult<()> {
        self.binary_op(gvs, |l, r| l < r, |l, r| l < r)?;
        Ok(())
    }

    fn equal(&mut self, gvs: &mut GVS) -> VMResult<Variable> {
        let right = self.acc.pop(gvs)?;
        let left = self.acc.pop(gvs)?;

        let variable = match (left.value_type, right.value_type) {
            (INT_TYPE, INT_TYPE) => Ok(Variable::from(left.value as i64 == right.value as i64)),
            (INT_TYPE, FLOAT_TYPE) | (FLOAT_TYPE, INT_TYPE) | (FLOAT_TYPE, FLOAT_TYPE) => {
                Ok(Variable::from(left.as_f64()? == right.as_f64()?))
            }
            (STRING_TYPE, STRING_TYPE) => {
                let left_string = gvs.collection_string(left.value)?;
                let right_string = gvs.collection_string(right.value)?;
                Ok(Variable::from(left_string == right_string))
            }
            _ => Err("Invalid type"),
        }?;

        Ok(variable)
    }

    fn eq(&mut self, gvs: &mut GVS) -> VMResult<()> {
        let variable = self.equal(gvs)?;
        self.acc.push(gvs, variable)?;
        Ok(())
    }

    fn not_eq(&mut self, gvs: &mut GVS) -> VMResult<()> {
        let variable = self.equal(gvs)?;
        let variable = Variable::from(!variable.as_boolean());
        self.acc.push(gvs, variable)?;

        Ok(())
    }

    fn bool_op(&mut self, gvs: &mut GVS, operation: impl Fn(bool, bool) -> bool) -> VMResult<()> {
        let right = self.acc.pop(gvs)?;
        let left = self.acc.pop(gvs)?;

        let variable = match (left.value_type, right.value_type) {
            (BOOLEAN_TYPE, BOOLEAN_TYPE) => Ok(Variable::from(operation(
                left.as_boolean(),
                right.as_boolean(),
            ))),
            _ => Err("Invalid type"),
        }?;

        self.acc.push(gvs, variable)?;

        Ok(())
    }

    fn and(&mut self, gvs: &mut GVS) -> VMResult<()> {
        self.bool_op(gvs, |l, r| l && r)?;
        Ok(())
    }

    fn or(&mut self, gvs: &mut GVS) -> VMResult<()> {
        self.bool_op(gvs, |l, r| l || r)?;
        Ok(())
    }

    fn not(&mut self, gvs: &mut GVS) -> VMResult<()> {
        let variable = self.acc.pop(gvs)?;

        match variable.value_type {
            BOOLEAN_TYPE => self.acc.push(gvs, Variable::from(!variable.as_boolean())),
            _ => Err(VMError::from("Invalid value_type for not operator")),
        }
    }

    fn increment(&mut self, gvs: &mut GVS) -> VMResult<()> {
        let variable = self.acc.last_mut(gvs)?;

        variable.value = match variable.value_type {
            INT_TYPE => {
                let mut value = variable.value as i64;
                value += 1;

                Ok(value as u64)
            }
            FLOAT_TYPE => {
                let mut value = variable.as_f64()?;
                value += 1.0;
                Ok(value.to_bits())
            }
            _ => Err("Invalid value_type for increment operator"),
        }?;

        Ok(())
    }

    fn decrement(&mut self, gvs: &mut GVS) -> VMResult<()> {
        let variable = self.acc.last_mut(gvs)?;

        variable.value = match variable.value_type {
            INT_TYPE => {
                let mut value = variable.value as i64;
                value -= 1;

                Ok(value as u64)
            }
            FLOAT_TYPE => {
                let mut value = variable.as_f64()?;
                value -= 1.0;
                Ok(value.to_bits())
            }
            _ => Err("Invalid value_type for decrement operator"),
        }?;

        Ok(())
    }

    fn clone_string(&mut self, gvs: &mut GVS, variable: &mut Variable) -> VMResult<()> {
        let collection_id = variable.value;
        let string = gvs.collection_string(collection_id)?;
        let collection_id = gvs.collection_store_string(string.to_string());

        variable.value = collection_id;
        Ok(())
    }

    fn clone_stack(&mut self, gvs: &mut GVS, variable: &mut Variable) -> VMResult<()> {
        let collection_id = variable.value;
        let stack = gvs.collection_stack(collection_id)?.to_vec();

        // Todo: Implement deep cloning for matrices
        let stack = stack
            .iter()
            .map(|storage_id| {
                let variable = gvs.variable(*storage_id)?.clone();
                let storage_id = gvs.store(variable);
                Ok(storage_id)
            })
            .collect::<VMResult<Vec<StorageId>>>()?;

        let collection_id = gvs.collection_store_stack(stack);
        variable.value = collection_id;

        Ok(())
    }

    fn clone(&mut self, gvs: &mut GVS) -> VMResult<()> {
        let mut variable = self.acc.pop(gvs)?;
        variable.owners = 0;

        match variable.value_type {
            INT_TYPE | FLOAT_TYPE | NULL_TYPE | BOOLEAN_TYPE => Ok(()),
            STRING_TYPE => self.clone_string(gvs, &mut variable),
            STACK_TYPE => self.clone_stack(gvs, &mut variable),
            _ => Err(VMError::from("Invalid value_type for clone")),
        }?;

        self.acc.push(gvs, variable)?;

        Ok(())
    }

    fn load_collection(&mut self, gvs: &mut GVS, size: usize) -> VMResult<()> {
        let stack = self.acc.size_pop(size);
        let collection_id = gvs.collection_store_stack(stack);

        self.acc.push(gvs, Variable::collection(collection_id))?;

        Ok(())
    }

    fn store(&mut self) -> VMResult<()> {
        if let Some(storage_id) = self.acc.data.pop() {
            self.stack.data.push(storage_id);
            Ok(())
        } else {
            Err(VMError::from("No storage_id in acc stack"))
        }
    }

    fn free(&mut self, gvs: &mut GVS, size: usize) -> VMResult<()> {
        for _ in 0..size {
            self.stack.free_last(gvs)?;
        }

        Ok(())
    }

    fn clear_acc(&mut self, gvs: &mut GVS) -> VMResult<()> {
        while let Some(storage_id) = self.acc.data.pop() {
            gvs.storage_remove_owner(storage_id)?;
        }

        Ok(())
    }

    fn jump_if(&mut self, gvs: &mut GVS, offset: i32, boolean: bool) -> VMResult<()> {
        let variable = self.acc.pop(gvs)?;

        if variable.value_type != BOOLEAN_TYPE {
            return Err(VMError::from("Invalid value type, expected boolean"));
        }

        if variable.as_boolean() == boolean {
            self.jump(offset)?;
        }

        Ok(())
    }

    fn call(&mut self, gvs: &mut GVS) -> VMResult<()> {
        let storage_id = self.acc.pop_data()?;

        let variable_function = gvs.variable(storage_id)?;
        let function = variable_function.as_function()?;

        self.prevent_step = true;

        let return_pointer = self.program_counter;
        let stack_pointer = self.stack.len() as Pointer;

        let call_stack = CallStack::new(return_pointer, stack_pointer, storage_id);
        self.call_stack.push(call_stack);

        self.program_counter = function.pointer as usize;

        Ok(())
    }

    fn on_return(&mut self, gvs: &mut GVS) -> VMResult<()> {
        if let Some(call_stack) = self.call_stack.pop() {
            gvs.storage_remove_owner(call_stack.storage_id)?;
            self.program_counter = call_stack.return_pointer;

            Ok(())
        } else {
            Err(VMError::from("CallStack is empty, cannot execute return"))
        }
    }

    fn load_function(&mut self, gvs: &mut GVS, captures: CaptureSize) -> VMResult<()> {
        let collection_id = if captures == 0 {
            None
        } else {
            let stack = self.acc.size_pop(captures);
            let collection_id = gvs.collection_store_stack(stack);

            Some(collection_id as u32)
        };

        let variable_pointer = self.acc.pop(gvs)?;

        let function = if let Some(collection_id) = collection_id {
            Function::new(variable_pointer.value as u32, collection_id)
        } else {
            Function::from(variable_pointer.value as u32)
        };

        let variable_function = Variable::from(function);

        self.acc.push(gvs, variable_function)?;

        Ok(())
    }

    fn last_function(&self, gvs: &mut GVS) -> VMResult<Function> {
        if let Some(call_stack) = self.call_stack.last() {
            let variable = gvs.variable(call_stack.storage_id)?;
            let function = variable.as_function()?;
            Ok(function)
        } else {
            Err(VMError::from("Call stack is empty"))
        }
    }

    fn load_capture(&mut self, gvs: &mut GVS, slot_id: StorageId) -> VMResult<()> {
        let function = self.last_function(gvs)?;

        let collection_id = function.collection_id()?;
        let collection = gvs.collection_stack(collection_id as CollectionId)?;

        if let Some(storage_id) = collection.get(slot_id as usize) {
            self.acc.push_storage_id(gvs, *storage_id)?;

            Ok(())
        } else {
            Err(VMError::from(format!(
                "The function does not have captured variable with slot_id {}",
                slot_id
            )))
        }
    }

    fn collection_len_stack(
        &mut self,
        gvs: &mut GVS,
        collection_id: CollectionId,
    ) -> VMResult<i64> {
        let collection_len = {
            let collection = gvs.collection_stack(collection_id)?;
            collection.len() as i64
        };

        Ok(collection_len)
    }

    fn collection_len_string(
        &mut self,
        gvs: &mut GVS,
        collection_id: CollectionId,
    ) -> VMResult<i64> {
        let collection_len = {
            let string = gvs.collection_string(collection_id)?;
            string.len() as i64
        };

        Ok(collection_len)
    }

    fn collection_len(&mut self, gvs: &mut GVS) -> VMResult<()> {
        let (collection_id, value_type) = {
            let variable = self.acc.last(gvs)?;

            (variable.value as CollectionId, variable.value_type)
        };

        let collection_len = match value_type {
            STACK_TYPE => self.collection_len_stack(gvs, collection_id),
            STRING_TYPE => self.collection_len_string(gvs, collection_id),
            _ => Err(VMError::from("Variable is not a stack!")),
        }?;

        self.acc.pop(gvs)?;

        let variable = Variable::from(collection_len);
        self.acc.push(gvs, variable)?;

        Ok(())
    }

    fn load_from_collection_stack(
        &mut self,
        gvs: &mut GVS,
        collection_id: CollectionId,
        index: usize,
    ) -> VMResult<()> {
        let collection = gvs.collection_stack(collection_id)?;

        if let Some(storage_id) = collection.get(index) {
            self.acc.push_storage_id(gvs, *storage_id)?;
            Ok(())
        } else {
            Err(VMError::from(format!("No value by that index {}", index)))
        }
    }

    fn load_from_collection_string(
        &mut self,
        gvs: &mut GVS,
        collection_id: CollectionId,
        index: usize,
    ) -> VMResult<()> {
        let collection = gvs.collection_string(collection_id)?;

        let string = collection.to_string();

        if let Some(char) = string.chars().collect::<Vec<char>>().get(index) {
            let char_string = format!("{}", char);
            let collection_id = gvs.collection_store_string(char_string);
            let string_variable = Variable::string(collection_id);

            self.acc.push(gvs, string_variable)?;

            Ok(())
        } else {
            Err(VMError::from(format!("No value by that index {}", index)))
        }
    }

    fn load_from_collection(&mut self, gvs: &mut GVS) -> VMResult<()> {
        let index_variable = self.acc.pop(gvs)?;
        if index_variable.value_type != INT_TYPE {
            return Err(VMError::from("Index variable is not an integer"));
        }

        let collection_variable = self.acc.pop(gvs)?;

        let collection_id = collection_variable.value;
        let index = index_variable.value as usize;

        match collection_variable.value_type {
            STACK_TYPE => self.load_from_collection_stack(gvs, collection_id, index),
            STRING_TYPE => self.load_from_collection_string(gvs, collection_id, index),
            _ => Err(VMError::from("This is not a collection")),
        }?;

        Ok(())
    }

    fn assign_for_variable(&mut self, gvs: &mut GVS, slot_id: StorageId) -> VMResult<()> {
        let slot_id = slot_id as usize;

        let storage_id = self.stack.data[slot_id];
        gvs.storage_remove_owner(storage_id)?;

        let new_storage_id = self.acc.pop_data()?;

        self.stack.data[slot_id] = new_storage_id;

        Ok(())
    }

    fn assign_for_collection(
        &mut self,
        gvs: &mut GVS,
        collection_id: CollectionId,
        index: usize,
    ) -> VMResult<()> {
        let storage_id = {
            let collection = gvs.collection_stack(collection_id)?;
            if let Some(storage_id) = collection.get(index) {
                Ok(*storage_id)
            } else {
                Err(VMError::from("No storage_id in collection"))
            }
        }?;

        gvs.storage_remove_owner(storage_id)?;

        let new_storage_id = self.acc.pop_data()?;

        let collection = gvs.collection_stack_mut(collection_id)?;
        collection[index] = new_storage_id;

        Ok(())
    }

    fn assign(&mut self, gvs: &mut GVS) -> VMResult<()> {
        match self.assign {
            Assign::Variable(slot_id) => self.assign_for_variable(gvs, slot_id),
            Assign::Collection(collection_id, index) => {
                self.assign_for_collection(gvs, collection_id, index)
            }
            Assign::None => Err(VMError::from("No assign available")),
        }?;

        self.assign = Assign::None;

        Ok(())
    }

    fn assign_variable(&mut self, slot_id: Slot) -> VMResult<()> {
        self.assign = Assign::Variable(slot_id);
        Ok(())
    }

    fn assign_collection_from_variable(
        &mut self,
        gvs: &mut GVS,
        slot_id: Slot,
        index: usize,
    ) -> VMResult<()> {
        let storage_id = self.stack.storage_id(slot_id)?;
        let variable = gvs.variable(storage_id)?;

        if variable.value_type != STACK_TYPE {
            return Err(VMError::from("Cannot extract slot_id from not stack"));
        }

        self.assign = Assign::Collection(variable.value, index);

        Ok(())
    }

    fn assign_collection_from_collection(
        &mut self,
        gvs: &mut GVS,
        collection_id: CollectionId,
        collection_index: usize,
        index: usize,
    ) -> VMResult<()> {
        let collection = gvs.collection_stack(collection_id)?;
        let storage_id = collection
            .get(collection_index)
            .ok_or_else(|| "No storage_id in collection")?;

        let variable = gvs.variable(*storage_id)?;

        if variable.value_type != STACK_TYPE {
            return Err(VMError::from("Cannot extract slot_id from not stack"));
        }

        self.assign = Assign::Collection(variable.value, index);

        Ok(())
    }

    fn assign_collection(&mut self, gvs: &mut GVS) -> VMResult<()> {
        let index_variable = self.acc.pop(gvs)?;
        let index = index_variable.value as usize;

        match self.assign {
            Assign::Variable(slot_id) => self.assign_collection_from_variable(gvs, slot_id, index),
            Assign::Collection(collection_id, collection_index) => {
                self.assign_collection_from_collection(gvs, collection_id, collection_index, index)
            }
            Assign::None => Err(VMError::from("No assign available for collection")),
        }?;

        Ok(())
    }

    fn call_native(
        &self,
        native_stack: &mut Vec<NativeCall>,
        native_id: NativeId,
        arguments: Arguments,
        runner_id: usize,
    ) -> VMResult<()> {
        let native_call = NativeCall::new(native_id, arguments, runner_id);
        native_stack.push(native_call);
        Ok(())
    }

    pub fn run(
        &mut self,
        runner_id: usize,
        instruction: Instruction,
        gvs: &mut GVS,
        native_stack: &mut Vec<NativeCall>,
    ) -> VMResult<()> {
        match instruction {
            Instruction::LoadConst(constant) => self.load_const(gvs, constant),
            Instruction::LoadVar(slot) => self.load_var(gvs, slot),
            Instruction::Jump(offset) => self.jump(offset),
            Instruction::Add => self.add(gvs),
            Instruction::Minus => self.minus(gvs),
            Instruction::Mul => self.mul(gvs),
            Instruction::Div => self.div(gvs),
            Instruction::Eq => self.eq(gvs),
            Instruction::GreaterEq => self.greater_eq(gvs),
            Instruction::Greater => self.greater(gvs),
            Instruction::LessEq => self.less_eq(gvs),
            Instruction::Less => self.less(gvs),
            Instruction::NotEq => self.not_eq(gvs),
            Instruction::And => self.and(gvs),
            Instruction::Or => self.or(gvs),
            Instruction::Not => self.not(gvs),
            Instruction::Increment => self.increment(gvs),
            Instruction::Decrement => self.decrement(gvs),
            Instruction::Clone => self.clone(gvs),
            Instruction::LoadCollection(size) => self.load_collection(gvs, size),
            Instruction::Store => self.store(),
            Instruction::Free(size) => self.free(gvs, size),
            Instruction::ClearAcc => self.clear_acc(gvs),
            Instruction::JumpIfFalse(offset) => self.jump_if(gvs, offset, false),
            Instruction::JumpIfTrue(offset) => self.jump_if(gvs, offset, true),
            Instruction::Call => self.call(gvs),
            Instruction::Return => self.on_return(gvs),
            Instruction::LoadFunction(captures) => self.load_function(gvs, captures),
            Instruction::LoadCapture(slot_id) => self.load_capture(gvs, slot_id),
            Instruction::CollectionLen => self.collection_len(gvs),
            Instruction::LoadFromCollection => self.load_from_collection(gvs),
            Instruction::Assign => self.assign(gvs),
            Instruction::AssignVariable(slot_id) => self.assign_variable(slot_id),
            Instruction::AssignCollection => self.assign_collection(gvs),
            Instruction::CallNative(native_id, arguments) => {
                self.call_native(native_stack, native_id, arguments, runner_id)
            }
        }?;

        self.step();

        Ok(())
    }

    pub fn program_counter(&self) -> Pointer {
        self.program_counter
    }
}
