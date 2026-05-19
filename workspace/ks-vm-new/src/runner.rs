use ks_global::utils::ks_error::KsError;
use ks_global::utils::ks_result::KsResult;

use super::types::{Offset, Pointer, Slot};
use super::{Constant, Instruction};

use crate::gvs::variable::{
    BOOLEAN_TYPE, COLLECTION_TYPE, FLOAT_TYPE, INT_TYPE, NULL_TYPE, STRING_TYPE,
};
use crate::gvs::{GVS, Stack, Variable};
use crate::types::{CollectionId, StorageId};

#[derive(Debug)]
pub struct Runner {
    pub program_counter: Pointer,
    pub acc: Stack,
    pub stack: Stack,
    pub prevent_step: bool,
}

impl Runner {
    pub fn new() -> Self {
        Self {
            program_counter: 0,
            acc: Stack::new(),
            stack: Stack::new(),
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

    fn load_const(&mut self, gvs: &mut GVS, constant: Constant) -> KsResult<()> {
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

    fn load_var(&mut self, gvs: &mut GVS, slot: Slot) -> KsResult<()> {
        let storage_id = self.stack.storage_id(slot)?;
        self.acc.push_storage_id(gvs, storage_id)?;

        Ok(())
    }

    fn jump(&mut self, offset: Offset) -> KsResult<()> {
        self.program_counter = if offset < 0 {
            self.program_counter
                .saturating_sub(offset.unsigned_abs() as usize)
        } else {
            self.program_counter.saturating_add(offset as usize)
        };

        self.prevent_step = true;

        Ok(())
    }

    fn add_strings(gvs: &mut GVS, left: CollectionId, right: CollectionId) -> KsResult<u64> {
        let mut left = gvs.collection_string(left)?.to_string();
        let right = gvs.collection_string(right)?;

        left.push_str(right);
        let collection_id = gvs.collection_store_string(left);

        Ok(collection_id)
    }

    fn add(&mut self, gvs: &mut GVS) -> KsResult<()> {
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
            _ => Err(KsError::runtime("Invalid type")),
        }?;

        self.acc.push(gvs, variable)?;

        Ok(())
    }

    fn binary_op<RI, RF>(
        &mut self,
        gvs: &mut GVS,
        operation_int: impl Fn(i64, i64) -> RI,
        operation_float: impl Fn(f64, f64) -> RF,
    ) -> KsResult<()>
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
            _ => Err(KsError::runtime("Invalid type")),
        }?;

        self.acc.push(gvs, variable)?;

        Ok(())
    }

    fn minus(&mut self, gvs: &mut GVS) -> KsResult<()> {
        self.binary_op(gvs, |l, r| l - r, |l, r| l - r)?;
        Ok(())
    }

    fn mul(&mut self, gvs: &mut GVS) -> KsResult<()> {
        self.binary_op(gvs, |l, r| l * r, |l, r| l * r)?;
        Ok(())
    }

    fn check_zero_division(&self, gvs: &mut GVS) -> KsResult<()> {
        let variable = self.acc.last(gvs)?;
        let float_value = variable.as_f64()?;

        if (variable.value_type == INT_TYPE || variable.value_type == FLOAT_TYPE)
            && float_value == 0.0
        {
            return Err(KsError::runtime("Zero division error"));
        }

        Ok(())
    }

    fn div(&mut self, gvs: &mut GVS) -> KsResult<()> {
        self.check_zero_division(gvs)?;
        let right = self.acc.pop(gvs)?;
        let left = self.acc.pop(gvs)?;

        let variable = match (left.value_type, right.value_type) {
            (INT_TYPE, INT_TYPE)
            | (INT_TYPE, FLOAT_TYPE)
            | (FLOAT_TYPE, INT_TYPE)
            | (FLOAT_TYPE, FLOAT_TYPE) => Ok(Variable::from(left.as_f64()? / right.as_f64()?)),
            _ => Err(KsError::runtime("Invalid type")),
        }?;

        self.acc.push(gvs, variable)?;

        Ok(())
    }

    fn greater_eq(&mut self, gvs: &mut GVS) -> KsResult<()> {
        self.binary_op(gvs, |l, r| l >= r, |l, r| l >= r)?;
        Ok(())
    }

    fn greater(&mut self, gvs: &mut GVS) -> KsResult<()> {
        self.binary_op(gvs, |l, r| l > r, |l, r| l > r)?;
        Ok(())
    }

    fn less_eq(&mut self, gvs: &mut GVS) -> KsResult<()> {
        self.binary_op(gvs, |l, r| l <= r, |l, r| l <= r)?;
        Ok(())
    }

    fn less(&mut self, gvs: &mut GVS) -> KsResult<()> {
        self.binary_op(gvs, |l, r| l < r, |l, r| l < r)?;
        Ok(())
    }

    fn equal(&mut self, gvs: &mut GVS) -> KsResult<Variable> {
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
            _ => Err(KsError::runtime("Invalid type")),
        }?;

        Ok(variable)
    }

    fn eq(&mut self, gvs: &mut GVS) -> KsResult<()> {
        let variable = self.equal(gvs)?;
        self.acc.push(gvs, variable)?;
        Ok(())
    }

    fn not_eq(&mut self, gvs: &mut GVS) -> KsResult<()> {
        let variable = self.equal(gvs)?;
        let variable = Variable::from(!variable.as_boolean());
        self.acc.push(gvs, variable)?;

        Ok(())
    }

    fn bool_op(&mut self, gvs: &mut GVS, operation: impl Fn(bool, bool) -> bool) -> KsResult<()> {
        let right = self.acc.pop(gvs)?;
        let left = self.acc.pop(gvs)?;

        let variable = match (left.value_type, right.value_type) {
            (BOOLEAN_TYPE, BOOLEAN_TYPE) => Ok(Variable::from(operation(
                left.as_boolean(),
                right.as_boolean(),
            ))),
            _ => Err(KsError::runtime("Invalid type")),
        }?;

        self.acc.push(gvs, variable)?;

        Ok(())
    }

    fn and(&mut self, gvs: &mut GVS) -> KsResult<()> {
        self.bool_op(gvs, |l, r| l && r)?;
        Ok(())
    }

    fn or(&mut self, gvs: &mut GVS) -> KsResult<()> {
        self.bool_op(gvs, |l, r| l || r)?;
        Ok(())
    }

    fn not(&mut self, gvs: &mut GVS) -> KsResult<()> {
        let variable = self.acc.pop(gvs)?;

        match variable.value_type {
            BOOLEAN_TYPE => self.acc.push(gvs, Variable::from(!variable.as_boolean())),
            _ => Err(KsError::runtime("Invalid value_type for not operator")),
        }
    }

    fn increment(&mut self, gvs: &mut GVS) -> KsResult<()> {
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
            _ => Err(KsError::runtime(
                "Invalid value_type for increment operator",
            )),
        }?;

        Ok(())
    }

    fn decrement(&mut self, gvs: &mut GVS) -> KsResult<()> {
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
            _ => Err(KsError::runtime(
                "Invalid value_type for decrement operator",
            )),
        }?;

        Ok(())
    }

    fn clone_string(&mut self, gvs: &mut GVS, variable: &mut Variable) -> KsResult<()> {
        let collection_id = variable.value;
        let string = gvs.collection_string(collection_id)?;
        let collection_id = gvs.collection_store_string(string.to_string());

        variable.value = collection_id;
        Ok(())
    }

    fn clone_stack(&mut self, gvs: &mut GVS, variable: &mut Variable) -> KsResult<()> {
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
            .collect::<KsResult<Vec<StorageId>>>()?;

        let collection_id = gvs.collection_store_stack(stack);
        variable.value = collection_id;

        Ok(())
    }

    fn clone(&mut self, gvs: &mut GVS) -> KsResult<()> {
        let mut variable = self.acc.pop(gvs)?;
        variable.owners = 0;

        match variable.value_type {
            INT_TYPE | FLOAT_TYPE | NULL_TYPE | BOOLEAN_TYPE => {}
            STRING_TYPE => self.clone_string(gvs, &mut variable)?,
            COLLECTION_TYPE => self.clone_stack(gvs, &mut variable)?,
            _ => unreachable!(),
        }

        self.acc.push(gvs, variable)?;

        Ok(())
    }

    fn load_collection(&mut self, gvs: &mut GVS, size: usize) -> KsResult<()> {
        let stack = self.acc.size_pop(size);
        let collection_id = gvs.collection_store_stack(stack);

        self.acc.push(gvs, Variable::collection(collection_id))?;

        Ok(())
    }

    fn store(&mut self) -> KsResult<()> {
        if let Some(storage_id) = self.acc.data.pop() {
            self.stack.data.push(storage_id);
            Ok(())
        } else {
            Err(KsError::runtime("No storage_id in acc stack"))
        }
    }

    pub fn run(&mut self, instruction: Instruction, gvs: &mut GVS) -> KsResult<()> {
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
            _ => todo!(),
        }?;

        self.step();

        Ok(())
    }

    pub fn program_counter(&self) -> Pointer {
        self.program_counter
    }
}
