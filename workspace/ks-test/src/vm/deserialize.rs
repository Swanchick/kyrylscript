use ks_vm_new::ir::instructions::{
    ADD, AND, ASC, ASN, ASV, CALL, CLR, CPY, DEC, DIV, EQ, FREE, GE, GT, INC, JNZ, JZ, LBF, LBT,
    LDC, LDCP, LDF, LDFC, LDFN, LDI, LDN, LDS, LDV, LE, LEN, LT, MUL, NCALL, NE, NOT, OR, RET, STR,
    SUB,
};
use ks_vm_new::{Constant, Instruction, Program, VMResult};

macro_rules! deserialize_instruction {
    ($test: ident, $instruction: expr, $opcode: expr) => {
        #[test]
        fn $test() -> VMResult<()> {
            let test_program = Program::new(vec![$instruction]);
            let buffer = vec![$opcode];
            let program = Program::deserialize(buffer)?;

            assert_eq!(program, test_program);

            Ok(())
        }
    };
}

macro_rules! deserialize_instructions {
    ($test: ident, $instruction: expr, $opcodes: expr) => {
        #[test]
        fn $test() -> VMResult<()> {
            let test_program = Program::new(vec![$instruction]);
            let buffer = $opcodes;
            let program = Program::deserialize(buffer)?;

            assert_eq!(program, test_program);

            Ok(())
        }
    };
}

deserialize_instruction!(add, Instruction::Add, ADD);
deserialize_instruction!(sub, Instruction::Minus, SUB);
deserialize_instruction!(mul, Instruction::Mul, MUL);
deserialize_instruction!(div, Instruction::Div, DIV);
deserialize_instruction!(inc, Instruction::Increment, INC);
deserialize_instruction!(dec, Instruction::Decrement, DEC);

deserialize_instruction!(eq, Instruction::Eq, EQ);
deserialize_instruction!(ne, Instruction::NotEq, NE);
deserialize_instruction!(gt, Instruction::Greater, GT);
deserialize_instruction!(ge, Instruction::GreaterEq, GE);
deserialize_instruction!(lt, Instruction::Less, LT);
deserialize_instruction!(le, Instruction::LessEq, LE);

deserialize_instruction!(and, Instruction::And, AND);
deserialize_instruction!(or, Instruction::Or, OR);
deserialize_instruction!(not, Instruction::Not, NOT);

deserialize_instruction!(ret, Instruction::Return, RET);

deserialize_instructions!(
    jz,
    Instruction::JumpIfFalse(10),
    vec![JZ, 0xA, 0x0, 0x0, 0x0]
);
deserialize_instructions!(
    jnz,
    Instruction::JumpIfTrue(16),
    vec![JNZ, 0x10, 0x0, 0x0, 0x0]
);
deserialize_instructions!(jmp, Instruction::Jump(100), vec![JNZ, 0x64, 0x0, 0x0, 0x0]);

deserialize_instruction!(cpy, Instruction::Clone, CPY);
deserialize_instruction!(clr, Instruction::ClearAcc, CLR);
deserialize_instructions!(free, Instruction::Free(10), vec![FREE, 0xA, 0x0, 0x0, 0x0]);
deserialize_instruction!(call, Instruction::Call, CALL);
deserialize_instructions!(
    ncall,
    Instruction::CallNative(1, 2),
    vec![NCALL, 0x1, 0x0, 0x0, 0x0, 0x2, 0x0, 0x0, 0x1]
);

deserialize_instructions!(
    ldi,
    Instruction::LoadConst(Constant::Integer(200)),
    vec![LDI, 0xC8, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0]
);
deserialize_instructions!(
    ldf,
    Instruction::LoadConst(Constant::Float(3.14)),
    vec![LDF, 0x1F, 0x85, 0xEB, 0x51, 0xB8, 0x1E, 0x09, 0x40]
);
deserialize_instruction!(lbt, Instruction::LoadConst(Constant::Boolean(true)), LBT);
deserialize_instruction!(lbf, Instruction::LoadConst(Constant::Boolean(false)), LBF);

#[test]
fn lds() -> VMResult<()> {
    let string = String::from("Hello World");
    let string_length = string.len() as u32;
    let mut string_length = string_length.to_le_bytes().to_vec();
    let mut string_bytes = string.clone().as_bytes().to_vec();
    let mut buffer = vec![LDS];

    let test_program = Program::new(vec![Instruction::LoadConst(Constant::String(
        String::from(string),
    ))]);

    buffer.append(&mut string_length);
    buffer.append(&mut string_bytes);

    let program = Program::deserialize(buffer)?;

    assert_eq!(program, test_program);

    Ok(())
}

deserialize_instruction!(ldn, Instruction::LoadConst(Constant::Null), LDN);

deserialize_instructions!(
    ldfn,
    Instruction::LoadFunction(5),
    vec![LDFN, 0x5, 0x0, 0x0, 0x0]
);

deserialize_instructions!(
    ldc,
    Instruction::LoadCollection(67),
    vec![LDC, 0x43, 0x0, 0x0, 0x0]
);

deserialize_instruction!(str, Instruction::Store, STR);
deserialize_instruction!(asn, Instruction::Assign, ASN);

deserialize_instructions!(
    asv,
    Instruction::AssignVariable(10),
    vec![ASV, 0xA, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0]
);

deserialize_instruction!(asc, Instruction::AssignCollection, ASC);

deserialize_instructions!(
    ldv,
    Instruction::LoadVar(10),
    vec![LDV, 0xA, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0]
);

deserialize_instructions!(
    ldcp,
    Instruction::LoadCapture(10),
    vec![LDCP, 0xA, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0]
);

deserialize_instruction!(ldfc, Instruction::LoadFromCollection, LDFC);
deserialize_instruction!(len, Instruction::CollectionLen, LEN);
