use ks_core::compiler_new::instructions::Instruction;
use ks_core::compiler_new::{constant::Constant, serializer::Serializer};

use ks_vm_new::ir::instructions::{
    ADD, AND, ASC, ASN, ASV, ASV8, ASV16, CALL, CALL8, CALL16, CLR, CPY, DEC, DIV, EQ, FREE, FREE8,
    FREE16, GE, GT, INC, JMP, JMP8, JMP16, JNZ, JNZ8, JNZ16, JZ, JZ8, JZ16, LBF, LBT, LDC, LDC8,
    LDC16, LDCP, LDCP8, LDCP16, LDF, LDFC, LDFN, LDFN8, LDFN16, LDI, LDI8, LDI16, LDI32, LDN, LDS,
    LDV, LDV8, LDV16, LE, LEN, LT, MUL, NCALL, NE, NOT, OR, RET, STR, SUB,
};

macro_rules! serialize_instruction {
    ($test: ident, $instruction: expr, $opcode: expr) => {
        #[test]
        fn $test() {
            let serializer = Serializer::new(vec![$instruction]);
            assert_eq!(serializer.serialize(), vec![$opcode]);
        }
    };
}

macro_rules! serialize_instructions {
    ($test: ident, $instruction: expr, $opcodes: expr) => {
        #[test]
        fn $test() {
            let serializer = Serializer::new(vec![$instruction]);
            let buffer = $opcodes;
            assert_eq!(serializer.serialize(), buffer);
        }
    };
}

serialize_instruction!(add, Instruction::Add, ADD);
serialize_instruction!(sub, Instruction::Minus, SUB);
serialize_instruction!(mul, Instruction::Mul, MUL);
serialize_instruction!(div, Instruction::Div, DIV);
serialize_instruction!(eq, Instruction::Eq, EQ);
serialize_instruction!(ne, Instruction::NotEq, NE);
serialize_instruction!(gt, Instruction::Greater, GT);
serialize_instruction!(ge, Instruction::GreaterEq, GE);
serialize_instruction!(lt, Instruction::Less, LT);
serialize_instruction!(not_eq, Instruction::NotEq, NE);
serialize_instruction!(greater, Instruction::Greater, GT);
serialize_instruction!(greater_eq, Instruction::GreaterEq, GE);
serialize_instruction!(less, Instruction::Less, LT);
serialize_instruction!(less_eq, Instruction::LessEq, LE);
serialize_instruction!(and, Instruction::And, AND);
serialize_instruction!(or, Instruction::Or, OR);
serialize_instruction!(not, Instruction::Not, NOT);
serialize_instruction!(increment, Instruction::Increment, INC);
serialize_instruction!(decrement, Instruction::Decrement, DEC);
serialize_instruction!(clone, Instruction::Clone, CPY);
serialize_instruction!(clear_acc, Instruction::ClearAcc, CLR);
serialize_instruction!(serialize_return, Instruction::Return, RET);

serialize_instructions!(free, Instruction::Free(u32::MAX as usize), {
    let mut expected = vec![FREE];
    expected.extend_from_slice(&u32::MAX.to_le_bytes());
    expected
});

serialize_instructions!(free8, Instruction::Free(u8::MAX as usize), {
    let mut expected = vec![FREE8];
    expected.extend_from_slice(&u8::MAX.to_le_bytes());
    expected
});

serialize_instructions!(free16, Instruction::Free(u16::MAX as usize), {
    let mut expected = vec![FREE16];
    expected.extend_from_slice(&u16::MAX.to_le_bytes());
    expected
});

serialize_instructions!(jump_if_false, Instruction::JumpIfFalse(i32::MAX), {
    let mut expected = vec![JZ];
    expected.extend_from_slice(&u32::MAX.to_le_bytes());
    expected
});

serialize_instructions!(jump_if_false8, Instruction::JumpIfFalse(i8::MAX as i32), {
    let mut expected = vec![JZ8];
    expected.extend_from_slice(&u8::MAX.to_le_bytes());
    expected
});

serialize_instructions!(
    jump_if_false16,
    Instruction::JumpIfFalse(i16::MAX as i32),
    {
        let mut expected = vec![JZ16];
        expected.extend_from_slice(&u16::MAX.to_le_bytes());
        expected
    }
);

serialize_instructions!(jump_if_true, Instruction::JumpIfTrue(i32::MAX), {
    let mut expected = vec![JNZ];
    expected.extend_from_slice(&u32::MAX.to_le_bytes());
    expected
});

serialize_instructions!(jump_if_true8, Instruction::JumpIfTrue(i8::MAX as i32), {
    let mut expected = vec![JNZ8];
    expected.extend_from_slice(&u8::MAX.to_le_bytes());
    expected
});

serialize_instructions!(jump_if_true16, Instruction::JumpIfTrue(i16::MAX as i32), {
    let mut expected = vec![JNZ16];
    expected.extend_from_slice(&u16::MAX.to_le_bytes());
    expected
});

serialize_instructions!(jump, Instruction::Jump(i32::MAX), {
    let mut expected = vec![JMP];
    expected.extend_from_slice(&u32::MAX.to_le_bytes());
    expected
});

serialize_instructions!(jump8, Instruction::Jump(i8::MAX as i32), {
    let mut expected = vec![JMP8];
    expected.extend_from_slice(&u8::MAX.to_le_bytes());
    expected
});

serialize_instructions!(jump16, Instruction::Jump(i16::MAX as i32), {
    let mut expected = vec![JMP16];
    expected.extend_from_slice(&u16::MAX.to_le_bytes());
    expected
});

serialize_instruction!(store, Instruction::Store, STR);
serialize_instruction!(assign, Instruction::Assign, ASN);

serialize_instructions!(assign_variable, Instruction::AssignVariable(5), {
    let mut expected = vec![ASV];
    expected.extend_from_slice(&[0x1, 0x5]);
    expected
});

serialize_instructions!(
    assign_variable8,
    Instruction::AssignVariable(u8::MAX as u32),
    {
        let mut expected = vec![ASV8];
        expected.extend_from_slice(&u8::MAX.to_le_bytes());
        expected
    }
);

serialize_instructions!(
    assign_variable16,
    Instruction::AssignVariable(u16::MAX as u32),
    {
        let mut expected = vec![ASV16];
        expected.extend_from_slice(&u16::MAX.to_le_bytes());
        expected
    }
);

serialize_instruction!(assign_collection, Instruction::AssignCollection, ASC);

serialize_instructions!(
    load_const_integer_1,
    Instruction::LoadConst(Constant::Integer(200)),
    {
        let mut expected = vec![LDI];
        expected.extend_from_slice(&[0x1, 0xC8]);
        expected
    }
);

serialize_instructions!(
    load_const_integer_2,
    Instruction::LoadConst(Constant::Integer(51400)),
    {
        let mut expected = vec![LDI];
        expected.extend_from_slice(&[0x2, 0xC8, 0xC8]);
        expected
    }
);

serialize_instructions!(
    load_const_integer_3,
    Instruction::LoadConst(Constant::Integer(13158600)),
    {
        let mut expected = vec![LDI];
        expected.extend_from_slice(&[0x3, 0xC8, 0xC8, 0xC8]);
        expected
    }
);

serialize_instructions!(
    load_const_integer_4,
    Instruction::LoadConst(Constant::Integer(3368601800)),
    {
        let mut expected = vec![LDI];
        expected.extend_from_slice(&[0x4, 0xC8, 0xC8, 0xC8, 0xC8]);
        expected
    }
);

serialize_instructions!(
    load_const_integer_5,
    Instruction::LoadConst(Constant::Integer(862362061000)),
    {
        let mut expected = vec![LDI];
        expected.extend_from_slice(&[0x5, 0xC8, 0xC8, 0xC8, 0xC8, 0xC8]);
        expected
    }
);

serialize_instructions!(
    load_const_integer_6,
    Instruction::LoadConst(Constant::Integer(220764687616200)),
    {
        let mut expected = vec![LDI];
        expected.extend_from_slice(&[0x6, 0xC8, 0xC8, 0xC8, 0xC8, 0xC8, 0xC8]);
        expected
    }
);

serialize_instructions!(
    load_const_integer_7,
    Instruction::LoadConst(Constant::Integer(56515760029747400)),
    {
        let mut expected = vec![LDI];
        expected.extend_from_slice(&[0x7, 0xC8, 0xC8, 0xC8, 0xC8, 0xC8, 0xC8, 0xC8]);
        expected
    }
);

serialize_instructions!(
    load_const_integer8,
    Instruction::LoadConst(Constant::Integer(u8::MAX as i64)),
    {
        let mut expected = vec![LDI8];
        expected.extend_from_slice(&u8::MAX.to_le_bytes());
        expected
    }
);

serialize_instructions!(
    load_const_integer16,
    Instruction::LoadConst(Constant::Integer(u16::MAX as i64)),
    {
        let mut expected = vec![LDI16];
        expected.extend_from_slice(&u16::MAX.to_le_bytes());
        expected
    }
);

serialize_instructions!(
    load_const_integer32,
    Instruction::LoadConst(Constant::Integer(u32::MAX as i64)),
    {
        let mut expected = vec![LDI32];
        expected.extend_from_slice(&u32::MAX.to_le_bytes());
        expected
    }
);

serialize_instructions!(
    load_const_float,
    Instruction::LoadConst(Constant::Float(42.5)),
    {
        let mut expected = vec![LDF];
        expected.extend_from_slice(&42.5f64.to_bits().to_le_bytes());
        expected
    }
);

serialize_instruction!(
    load_const_true,
    Instruction::LoadConst(Constant::Boolean(true)),
    LBT
);

serialize_instruction!(
    load_const_false,
    Instruction::LoadConst(Constant::Boolean(false)),
    LBF
);

serialize_instruction!(load_const_null, Instruction::LoadConst(Constant::Null), LDN);

serialize_instructions!(
    load_const_string,
    Instruction::LoadConst(Constant::String("hello".into())),
    {
        let mut expected = vec![LDS];
        expected.extend_from_slice(&5u32.to_le_bytes());
        expected.extend_from_slice(b"hello");
        expected
    }
);

serialize_instructions!(load_var, Instruction::LoadVar(7), {
    let mut expected = vec![LDV];
    expected.extend_from_slice(&[0x1, 0x7]);
    expected
});

serialize_instructions!(load_var8, Instruction::LoadVar(u8::MAX as u32), {
    let mut expected = vec![LDV8];
    expected.extend_from_slice(&u8::MAX.to_le_bytes());
    expected
});

serialize_instructions!(load_var16, Instruction::LoadVar(u16::MAX as u32), {
    let mut expected = vec![LDV16];
    expected.extend_from_slice(&u16::MAX.to_le_bytes());
    expected
});

serialize_instructions!(call, Instruction::Call(u32::MAX as u32), {
    let mut expected = vec![CALL];
    expected.extend_from_slice(&u32::MAX.to_le_bytes());
    expected
});

serialize_instructions!(call8, Instruction::Call(u8::MAX as u32), {
    let mut expected = vec![CALL8];
    expected.extend_from_slice(&u8::MAX.to_le_bytes());
    expected
});

serialize_instructions!(call16, Instruction::Call(u16::MAX as u32), {
    let mut expected = vec![CALL16];
    expected.extend_from_slice(&u16::MAX.to_le_bytes());
    expected
});

serialize_instructions!(call_native, Instruction::CallNative(1, 2), {
    let mut expected = vec![NCALL];
    expected.extend_from_slice(&1u32.to_le_bytes());
    expected.extend_from_slice(&2u32.to_le_bytes());
    expected
});

serialize_instructions!(load_capture, Instruction::LoadCapture(9), {
    let mut expected = vec![LDCP];
    expected.extend_from_slice(&[0x1, 0x9]);
    expected
});

serialize_instructions!(load_capture8, Instruction::LoadCapture(u8::MAX as u32), {
    let mut expected = vec![LDCP8];
    expected.extend_from_slice(&u8::MAX.to_le_bytes());
    expected
});

serialize_instructions!(load_capture16, Instruction::LoadCapture(u16::MAX as u32), {
    let mut expected = vec![LDCP16];
    expected.extend_from_slice(&u16::MAX.to_le_bytes());
    expected
});

serialize_instructions!(
    load_function,
    Instruction::LoadFunction(u32::MAX as usize),
    {
        let mut expected = vec![LDFN];
        expected.extend_from_slice(&u32::MAX.to_le_bytes());
        expected
    }
);

serialize_instructions!(
    load_function8,
    Instruction::LoadFunction(u8::MAX as usize),
    {
        let mut expected = vec![LDFN8];
        expected.extend_from_slice(&u8::MAX.to_le_bytes());
        expected
    }
);

serialize_instructions!(
    load_function16,
    Instruction::LoadFunction(u16::MAX as usize),
    {
        let mut expected = vec![LDFN16];
        expected.extend_from_slice(&u16::MAX.to_le_bytes());
        expected
    }
);

serialize_instructions!(
    load_collection,
    Instruction::LoadCollection(u32::MAX as usize),
    {
        let mut expected = vec![LDC];
        expected.extend_from_slice(&u32::MAX.to_le_bytes());
        expected
    }
);

serialize_instructions!(
    load_collection8,
    Instruction::LoadCollection(u8::MAX as usize),
    {
        let mut expected = vec![LDC8];
        expected.extend_from_slice(&u8::MAX.to_le_bytes());
        expected
    }
);

serialize_instructions!(
    load_collection16,
    Instruction::LoadCollection(u16::MAX as usize),
    {
        let mut expected = vec![LDC16];
        expected.extend_from_slice(&u16::MAX.to_le_bytes());
        expected
    }
);

serialize_instruction!(load_from_collection, Instruction::LoadFromCollection, LDFC);
serialize_instruction!(collection_len, Instruction::CollectionLen, LEN);
