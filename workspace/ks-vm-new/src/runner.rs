#[cfg(not(feature = "std"))]
use alloc::format;
#[cfg(not(feature = "std"))]
use alloc::string::ToString;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use crate::data_size::{DWORD, DataSize32, DataSize64, INSTRUCTION, QWORD};
use crate::ir::byte_reader::ByteReader;
use crate::ir::instructions::{
    ADD, AND, ASC, ASN, ASV, ASV8, ASV16, CALL, CALL8, CALL16, CLR, CPY, DEC, DIV, EQ, FREE, FREE8,
    FREE16, GE, GT, INC, JMP, JMP8, JMP16, JNZ, JNZ8, JNZ16, JZ, JZ8, JZ16, LBF, LBT, LDC, LDC8,
    LDC16, LDCP, LDCP8, LDCP16, LDF, LDFC, LDFN, LDI, LDI8, LDI16, LDI32, LDN, LDS, LDV, LDV8,
    LDV16, LE, LEN, LT, MUL, NCALL, NE, NOT, OR, RET, STR, SUB,
};
use crate::{Assign, Function, NativeCall, VMError, VMHelper, VMResult};

use super::call_stack::CallStack;
use super::environment::variable::{
    BOOLEAN_TYPE, FLOAT_TYPE, INT_TYPE, NULL_TYPE, STACK_TYPE, STRING_TYPE,
};
use super::environment::{GVS, Stack, Variable};
use super::types::{CollectionId, Pointer, Slot, StorageId};

#[derive(Debug)]
pub struct Runner {
    pub pc: Pointer,
    pub acc: Stack,
    pub stack: Stack,
    pub call_stack: Vec<CallStack>,
    pub assign: Assign,
}

impl Default for Runner {
    fn default() -> Self {
        Self::new()
    }
}

impl Runner {
    pub fn new() -> Self {
        Self {
            pc: 0,
            acc: Stack::new(),
            stack: Stack::new(),
            call_stack: Vec::new(),
            assign: Assign::None,
        }
    }

    fn step(&mut self, steps: isize) -> VMResult<()> {
        self.pc = self
            .pc
            .checked_add_signed(steps)
            .ok_or("Stepped out of program memory")?;

        Ok(())
    }

    fn load_null(&mut self, gvs: &mut GVS) -> VMResult<()> {
        self.acc.push(gvs, Variable::null())?;
        self.step(1)
    }

    fn load_true(&mut self, gvs: &mut GVS) -> VMResult<()> {
        self.acc.push(gvs, Variable::from(true))?;
        self.step(1)
    }

    fn load_false(&mut self, gvs: &mut GVS) -> VMResult<()> {
        self.acc.push(gvs, Variable::from(false))?;
        self.step(1)
    }

    fn load_integer(
        &mut self,
        gvs: &mut GVS,
        reader: ByteReader,
        data_size: DataSize64,
    ) -> VMResult<()> {
        let number = match data_size {
            DataSize64::Byte => reader.parse_i8()? as i64,
            DataSize64::Word => reader.parse_i16()? as i64,
            DataSize64::DWord => reader.parse_i32()? as i64,
            DataSize64::QWord => reader.parse_i64()? as i64,
        };

        let variable = Variable::from(number);
        self.acc.push(gvs, variable)?;
        self.step(data_size.instruction_size())
    }

    fn load_float(&mut self, gvs: &mut GVS, reader: ByteReader) -> VMResult<()> {
        let number = f64::from_bits(reader.parse_u64()?);
        let variable = Variable::from(number);
        self.acc.push(gvs, variable)?;
        self.step(INSTRUCTION + QWORD)
    }

    fn load_string(&mut self, gvs: &mut GVS, mut reader: ByteReader) -> VMResult<()> {
        let size = reader.parse_u32()? as usize;
        self.step(INSTRUCTION + DWORD)?;
        reader.pc = self.pc;
        let string = reader.parse_string(size)?;

        let collection_id = gvs.collection_store_string(string.to_string());
        let variable = Variable::string(collection_id);
        self.acc.push(gvs, variable)?;

        self.step(size as isize)
    }

    fn load_var(
        &mut self,
        gvs: &mut GVS,
        reader: ByteReader,
        data_size: DataSize32,
    ) -> VMResult<()> {
        let padding = if let Some(call_stack) = self.call_stack.last() {
            call_stack.stack_pointer
        } else {
            0
        } as u32;

        let slot = padding + reader.from_data_size_32(&data_size)? as u32;

        let storage_id = self.stack.storage_id(slot)?;
        self.acc.push_storage_id(gvs, storage_id)?;

        self.step(data_size.instruction_size())
    }

    fn jump(&mut self, reader: ByteReader, data_size: DataSize32) -> VMResult<()> {
        let offset = match data_size {
            DataSize32::Byte => reader.parse_i8()? as isize,
            DataSize32::Word => reader.parse_i16()? as isize,
            DataSize32::DWord => reader.parse_i32()? as isize,
        };

        self.pc = self
            .pc
            .checked_add_signed(offset)
            .ok_or("Out of program bounding")?;

        Ok(())
    }

    fn add_strings(
        gvs: &mut GVS,
        left: CollectionId,
        right: CollectionId,
    ) -> VMResult<CollectionId> {
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
                let collection_id = Self::add_strings(
                    gvs,
                    left.value as CollectionId,
                    right.value as CollectionId,
                )?;
                Ok(Variable::string(collection_id))
            }
            _ => Err("Invalid type"),
        }?;

        self.acc.push(gvs, variable)?;

        self.step(INSTRUCTION)
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

        self.step(INSTRUCTION)
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

        self.step(INSTRUCTION)
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
                let left_string = gvs.collection_string(left.value as CollectionId)?;
                let right_string = gvs.collection_string(right.value as CollectionId)?;
                Ok(Variable::from(left_string == right_string))
            }
            _ => Err("Invalid type"),
        }?;

        self.step(INSTRUCTION)?;
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
        self.step(INSTRUCTION)
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
        }?;

        self.step(INSTRUCTION)
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

        self.step(INSTRUCTION)
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

        self.step(INSTRUCTION)
    }

    fn clone_string(&mut self, gvs: &mut GVS, variable: &mut Variable) -> VMResult<()> {
        let collection_id = variable.value as CollectionId;
        let string = gvs.collection_string(collection_id)?;
        let collection_id = gvs.collection_store_string(string.to_string());

        variable.value = collection_id as u64;
        Ok(())
    }

    fn clone_stack(&mut self, gvs: &mut GVS, variable: &mut Variable) -> VMResult<()> {
        let collection_id = variable.value as CollectionId;
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
        variable.value = collection_id as u64;

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
        self.step(INSTRUCTION)
    }

    fn load_collection(
        &mut self,
        gvs: &mut GVS,
        reader: ByteReader,
        data_size: DataSize32,
    ) -> VMResult<()> {
        let size = reader.from_data_size_32(&data_size)? as u32;

        let stack = self.acc.size_pop(size);
        let collection_id = gvs.collection_store_stack(stack);

        self.acc.push(gvs, Variable::collection(collection_id))?;

        self.step(data_size.instruction_size())
    }

    fn store(&mut self) -> VMResult<()> {
        let storage_id = self.acc.data.pop().ok_or("No storage_id in acc stack")?;
        self.stack.data.push(storage_id);
        self.step(INSTRUCTION)
    }

    fn free(&mut self, gvs: &mut GVS, reader: ByteReader, data_size: DataSize32) -> VMResult<()> {
        let size = reader.from_data_size_32(&data_size)?;

        for _ in 0..size {
            self.stack.free_last(gvs)?;
        }

        self.step(data_size.instruction_size())
    }

    fn clear_acc(&mut self, gvs: &mut GVS) -> VMResult<()> {
        while let Some(storage_id) = self.acc.data.pop() {
            gvs.storage_remove_owner(storage_id)?;
        }

        self.step(INSTRUCTION)
    }

    fn jump_if(
        &mut self,
        gvs: &mut GVS,
        reader: ByteReader,
        data_size: DataSize32,
        boolean: bool,
    ) -> VMResult<()> {
        let variable = self.acc.pop(gvs)?;

        if variable.value_type != BOOLEAN_TYPE {
            return Err(VMError::from("Invalid value type, expected boolean"));
        }

        if variable.as_boolean() == boolean {
            self.jump(reader, data_size)
        } else {
            self.step(data_size.instruction_size())
        }
    }

    fn call(&mut self, gvs: &mut GVS, reader: ByteReader, data_size: DataSize32) -> VMResult<()> {
        let arguments = reader.from_data_size_32(&data_size)?;

        self.step(data_size.instruction_size())?;

        let slot = self.acc.len() - arguments - 1;
        let storage_id = self.acc.remove(slot);

        let variable = gvs.variable(storage_id)?;
        let function = variable.as_function()?;

        let return_pointer = self.pc;
        let stack_pointer = self.stack.len() as Pointer;

        let call_stack = CallStack::new(return_pointer, stack_pointer, storage_id);
        self.call_stack.push(call_stack);

        self.pc = function.pointer as usize;

        Ok(())
    }

    fn on_return(&mut self, gvs: &mut GVS) -> VMResult<()> {
        let call_stack = self
            .call_stack
            .pop()
            .ok_or("CallStack is empty, cannot execute return")?;

        gvs.storage_remove_owner(call_stack.storage_id)?;
        self.pc = call_stack.return_pointer;

        Ok(())
    }

    fn load_function(&mut self, gvs: &mut GVS, reader: ByteReader) -> VMResult<()> {
        let (pointer, captures) = reader.parse_dual()?;

        let collection_id = if captures == 0 {
            None
        } else {
            let stack = self.acc.size_pop(captures);
            let collection_id = gvs.collection_store_stack(stack);

            Some(collection_id as u32)
        };

        let function = if let Some(collection_id) = collection_id {
            Function::new(pointer as u32, collection_id)
        } else {
            Function::from(pointer as u32)
        };

        let variable_function = Variable::from(function);
        self.acc.push(gvs, variable_function)?;
        self.step(INSTRUCTION + DWORD * 2)?;

        Ok(())
    }

    fn last_function(&self, gvs: &mut GVS) -> VMResult<Function> {
        let call_stack = self.call_stack.last().ok_or("Call stack is empty")?;
        let variable = gvs.variable(call_stack.storage_id)?;
        let function = variable.as_function()?;
        Ok(function)
    }

    fn load_capture(
        &mut self,
        gvs: &mut GVS,
        reader: ByteReader,
        data_size: DataSize32,
    ) -> VMResult<()> {
        let slot_id = reader.from_data_size_32(&data_size)?;

        let function = self.last_function(gvs)?;

        let collection_id = function.collection_id()?;
        let collection = gvs.collection_stack(collection_id as CollectionId)?;

        let storage_id = collection.get(slot_id).ok_or(format!(
            "The function does not have captured variable with slot_id {}",
            slot_id
        ))?;

        self.acc.push_storage_id(gvs, *storage_id)?;

        self.step(data_size.instruction_size())
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

        self.step(INSTRUCTION)
    }

    fn load_from_collection_stack(
        &mut self,
        gvs: &mut GVS,
        collection_id: CollectionId,
        index: usize,
    ) -> VMResult<()> {
        let collection = gvs.collection_stack(collection_id)?;

        let storage_id = collection
            .get(index)
            .ok_or(format!("No value by that index {}", index))?;

        self.acc.push_storage_id(gvs, *storage_id)?;
        Ok(())
    }

    fn load_from_collection_string(
        &mut self,
        gvs: &mut GVS,
        collection_id: CollectionId,
        index: usize,
    ) -> VMResult<()> {
        let collection = gvs.collection_string(collection_id)?;

        let string = collection.to_string();

        let char = *string
            .chars()
            .collect::<Vec<char>>()
            .get(index)
            .ok_or(format!("No value by that index {}", index))?;

        let char_string = format!("{}", char);
        let collection_id = gvs.collection_store_string(char_string);
        let string_variable = Variable::string(collection_id);

        self.acc.push(gvs, string_variable)?;

        Ok(())
    }

    fn load_from_collection(&mut self, gvs: &mut GVS) -> VMResult<()> {
        let index_variable = self.acc.pop(gvs)?;
        if index_variable.value_type != INT_TYPE {
            return Err(VMError::from("Index variable is not an integer"));
        }

        let collection_variable = self.acc.pop(gvs)?;

        let collection_id = collection_variable.value as CollectionId;
        let index = index_variable.value as usize;

        match collection_variable.value_type {
            STACK_TYPE => self.load_from_collection_stack(gvs, collection_id, index),
            STRING_TYPE => self.load_from_collection_string(gvs, collection_id, index),
            _ => Err(VMError::from("This is not a collection")),
        }?;

        self.step(INSTRUCTION)
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
        self.step(INSTRUCTION)
    }

    fn assign_variable(&mut self, reader: ByteReader, data_size: DataSize32) -> VMResult<()> {
        let slot_id = reader.from_data_size_32(&data_size)? as u32;
        self.assign = Assign::Variable(slot_id);
        self.step(data_size.instruction_size())
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

        let collection_id = variable.value as CollectionId;

        self.assign = Assign::Collection(collection_id, index);

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

        let collection_id = variable.value as CollectionId;
        self.assign = Assign::Collection(collection_id, index);

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

        self.step(INSTRUCTION)
    }

    fn call_native(
        &mut self,
        native_stack: &mut Vec<NativeCall>,
        runner_id: usize,
        reader: ByteReader,
    ) -> VMResult<()> {
        let (native_id, arguments) = reader.parse_dual()?;
        let native_call = NativeCall::new(native_id, arguments, runner_id);
        native_stack.push(native_call);
        self.step(INSTRUCTION + DWORD * 2)?;
        Ok(())
    }

    pub fn run<'a>(&mut self, helper: VMHelper<'a>) -> VMResult<()> {
        let gvs = helper.gvs;
        let reader = ByteReader::new(self.pc, helper.instructions);

        match helper.instruction {
            LDN => self.load_null(gvs),
            LBT => self.load_true(gvs),
            LBF => self.load_false(gvs),
            LDI8 => self.load_integer(gvs, reader, DataSize64::Byte),
            LDI16 => self.load_integer(gvs, reader, DataSize64::Word),
            LDI32 => self.load_integer(gvs, reader, DataSize64::DWord),
            LDI => self.load_integer(gvs, reader, DataSize64::QWord),
            LDF => self.load_float(gvs, reader),
            LDS => self.load_string(gvs, reader),
            LDV8 => self.load_var(gvs, reader, DataSize32::Byte),
            LDV16 => self.load_var(gvs, reader, DataSize32::Word),
            LDV => self.load_var(gvs, reader, DataSize32::DWord),
            JMP8 => self.jump(reader, DataSize32::Byte),
            JMP16 => self.jump(reader, DataSize32::Word),
            JMP => self.jump(reader, DataSize32::DWord),
            ADD => self.add(gvs),
            SUB => self.minus(gvs),
            MUL => self.mul(gvs),
            DIV => self.div(gvs),
            EQ => self.eq(gvs),
            GE => self.greater_eq(gvs),
            GT => self.greater(gvs),
            LE => self.less_eq(gvs),
            LT => self.less(gvs),
            NE => self.not_eq(gvs),
            AND => self.and(gvs),
            OR => self.or(gvs),
            NOT => self.not(gvs),
            INC => self.increment(gvs),
            DEC => self.decrement(gvs),
            CPY => self.clone(gvs),
            LDC8 => self.load_collection(gvs, reader, DataSize32::Byte),
            LDC16 => self.load_collection(gvs, reader, DataSize32::Word),
            LDC => self.load_collection(gvs, reader, DataSize32::DWord),
            STR => self.store(),
            FREE8 => self.free(gvs, reader, DataSize32::Byte),
            FREE16 => self.free(gvs, reader, DataSize32::Word),
            FREE => self.free(gvs, reader, DataSize32::DWord),
            CLR => self.clear_acc(gvs),
            JZ8 => self.jump_if(gvs, reader, DataSize32::Byte, false),
            JZ16 => self.jump_if(gvs, reader, DataSize32::Word, false),
            JZ => self.jump_if(gvs, reader, DataSize32::DWord, false),
            JNZ8 => self.jump_if(gvs, reader, DataSize32::Byte, true),
            JNZ16 => self.jump_if(gvs, reader, DataSize32::Word, true),
            JNZ => self.jump_if(gvs, reader, DataSize32::DWord, true),
            CALL8 => self.call(gvs, reader, DataSize32::Byte),
            CALL16 => self.call(gvs, reader, DataSize32::Word),
            CALL => self.call(gvs, reader, DataSize32::DWord),
            RET => self.on_return(gvs),
            LDFN => self.load_function(gvs, reader),
            LDCP8 => self.load_capture(gvs, reader, DataSize32::Byte),
            LDCP16 => self.load_capture(gvs, reader, DataSize32::Word),
            LDCP => self.load_capture(gvs, reader, DataSize32::DWord),
            LEN => self.collection_len(gvs),
            LDFC => self.load_from_collection(gvs),
            ASN => self.assign(gvs),
            ASV8 => self.assign_variable(reader, DataSize32::Byte),
            ASV16 => self.assign_variable(reader, DataSize32::Word),
            ASV => self.assign_variable(reader, DataSize32::DWord),
            ASC => self.assign_collection(gvs),
            NCALL => self.call_native(helper.native_stack, helper.runner_id, reader),
            opcode => Err(VMError::from(format!("Unknown instruction {:X}", opcode))),
        }?;

        Ok(())
    }
}
