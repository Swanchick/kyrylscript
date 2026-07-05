macro_rules! operation {
    ($operation:ident, $instruction:expr, $op:tt) => {
        paste! {
            #[test]
            fn [<$operation _int_int>]() -> VMResult<()> {
                let int_left = 10;
                let int_right = 10;

                let mut variable_left = Variable::from(int_left);
                variable_left.owners = 2;
                let mut variable_right = Variable::from(int_right);
                variable_right.owners = 2;
                let mut variable_result = Variable::from(int_left $op int_right);
                variable_result.owners = 1;

                KsDriver::operation_test(variable_left, variable_right, variable_result, $instruction)?;

                Ok(())
            }
        }
        paste! {
            #[test]
            fn [<$operation _int_float>]() -> VMResult<()> {
                let int_left = 10;
                let float_right = 3.14;

                let mut variable_left = Variable::from(int_left);
                variable_left.owners = 2;
                let mut variable_right = Variable::from(float_right);
                variable_right.owners = 2;
                let mut variable_result = Variable::from((int_left as f64) $op float_right);
                variable_result.owners = 1;

                KsDriver::operation_test(variable_left, variable_right, variable_result, $instruction)?;

                Ok(())
            }
        }

        paste! {
            #[test]
            fn [<$operation _float_int>]() -> VMResult<()> {
                let float_left = 3.14;
                let int_right = 10;

                let mut variable_left = Variable::from(float_left);
                variable_left.owners = 2;
                let mut variable_right = Variable::from(int_right);
                variable_right.owners = 2;
                let mut variable_result = Variable::from(float_left $op (int_right as f64));
                variable_result.owners = 1;

                KsDriver::operation_test(variable_left, variable_right, variable_result, $instruction)?;

                Ok(())
            }
        }

        paste! {
            #[test]
            fn [<$operation _float_float>]() -> VMResult<()> {
                let float_left = 3.14;
                let float_right = 1.23;

                let mut variable_left = Variable::from(float_left);
                variable_left.owners = 2;
                let mut variable_right = Variable::from(float_right);
                variable_right.owners = 2;
                let mut variable_result = Variable::from(float_left $op float_right);
                variable_result.owners = 1;

                KsDriver::operation_test(variable_left, variable_right, variable_result, $instruction)?;

                Ok(())
            }
        }
    };
}

pub(crate) use operation;
