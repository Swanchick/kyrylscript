use core::global::utils::ks_error::KsError;

use core::global::utils::ks_result::KsResult;
use core::vm::variable::Variable;
use core::vm::value::Value;



pub fn ks_len(args: Vec<Variable>) -> KsResult<Variable> {
    if args.len() > 1 {
        return Err(KsError::runtime("Too many arguments!"));
    }

    todo!()
}