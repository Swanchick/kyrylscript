use ks_vm_new::ir::instructions::{
    ADD, AND, ASC, ASN, ASV, CALL, CLR, CPY, DEC, DIV, EQ, FREE, GE, GT, INC, JMP, JNZ, JZ, LBF,
    LBT, LDC, LDCP, LDF, LDFC, LDFN, LDI, LDN, LDS, LDV, LE, LEN, LT, MUL, NCALL, NE, NOT, OR, RET,
    STR, SUB,
};
use ks_vm_new::{Constant, Instruction};

#[test]
fn serialize_add() {
    assert_eq!(Instruction::Add.to_bytes(), vec![ADD]);
}

#[test]
fn serialize_minus() {
    assert_eq!(Instruction::Minus.to_bytes(), vec![SUB]);
}

#[test]
fn serialize_mul() {
    assert_eq!(Instruction::Mul.to_bytes(), vec![MUL]);
}

#[test]
fn serialize_div() {
    assert_eq!(Instruction::Div.to_bytes(), vec![DIV]);
}

#[test]
fn serialize_eq() {
    assert_eq!(Instruction::Eq.to_bytes(), vec![EQ]);
}

#[test]
fn serialize_not_eq() {
    assert_eq!(Instruction::NotEq.to_bytes(), vec![NE]);
}

#[test]
fn serialize_greater() {
    assert_eq!(Instruction::Greater.to_bytes(), vec![GT]);
}

#[test]
fn serialize_greater_eq() {
    assert_eq!(Instruction::GreaterEq.to_bytes(), vec![GE]);
}

#[test]
fn serialize_less() {
    assert_eq!(Instruction::Less.to_bytes(), vec![LT]);
}

#[test]
fn serialize_less_eq() {
    assert_eq!(Instruction::LessEq.to_bytes(), vec![LE]);
}

#[test]
fn serialize_and() {
    assert_eq!(Instruction::And.to_bytes(), vec![AND]);
}

#[test]
fn serialize_or() {
    assert_eq!(Instruction::Or.to_bytes(), vec![OR]);
}

#[test]
fn serialize_not() {
    assert_eq!(Instruction::Not.to_bytes(), vec![NOT]);
}

#[test]
fn serialize_increment() {
    assert_eq!(Instruction::Increment.to_bytes(), vec![INC]);
}

#[test]
fn serialize_decrement() {
    assert_eq!(Instruction::Decrement.to_bytes(), vec![DEC]);
}

#[test]
fn serialize_clone() {
    assert_eq!(Instruction::Clone.to_bytes(), vec![CPY]);
}

#[test]
fn serialize_clear_acc() {
    assert_eq!(Instruction::ClearAcc.to_bytes(), vec![CLR]);
}

#[test]
fn serialize_return() {
    assert_eq!(Instruction::Return.to_bytes(), vec![RET]);
}

#[test]
fn serialize_free() {
    let mut expected = vec![FREE];
    expected.extend_from_slice(&42u64.to_le_bytes());

    assert_eq!(Instruction::Free(42).to_bytes(), expected);
}

#[test]
fn serialize_jump_if_false() {
    let mut expected = vec![JNZ];
    expected.extend_from_slice(&123u32.to_le_bytes());

    assert_eq!(Instruction::JumpIfFalse(123).to_bytes(), expected);
}

#[test]
fn serialize_jump_if_true() {
    let mut expected = vec![JZ];
    expected.extend_from_slice(&123u32.to_le_bytes());

    assert_eq!(Instruction::JumpIfTrue(123).to_bytes(), expected);
}

#[test]
fn serialize_jump() {
    let mut expected = vec![JMP];
    expected.extend_from_slice(&123u32.to_le_bytes());

    assert_eq!(Instruction::Jump(123).to_bytes(), expected);
}

#[test]
fn serialize_store() {
    assert_eq!(Instruction::Store.to_bytes(), vec![STR]);
}

#[test]
fn serialize_assign() {
    assert_eq!(Instruction::Assign.to_bytes(), vec![ASN]);
}

#[test]
fn serialize_assign_variable() {
    let mut expected = vec![ASV];
    expected.extend_from_slice(&5u64.to_le_bytes());

    assert_eq!(Instruction::AssignVariable(5).to_bytes(), expected);
}

#[test]
fn serialize_assign_collection() {
    assert_eq!(Instruction::AssignCollection.to_bytes(), vec![ASC]);
}

#[test]
fn serialize_load_const_integer() {
    let mut expected = vec![LDI];
    expected.extend_from_slice(&42u64.to_le_bytes());

    assert_eq!(
        Instruction::LoadConst(Constant::Integer(42)).to_bytes(),
        expected
    );
}

#[test]
fn serialize_load_const_float() {
    let mut expected = vec![LDF];
    expected.extend_from_slice(&42.5f64.to_bits().to_le_bytes());

    assert_eq!(
        Instruction::LoadConst(Constant::Float(42.5)).to_bytes(),
        expected
    );
}

#[test]
fn serialize_load_const_true() {
    assert_eq!(
        Instruction::LoadConst(Constant::Boolean(true)).to_bytes(),
        vec![LBT]
    );
}

#[test]
fn serialize_load_const_false() {
    assert_eq!(
        Instruction::LoadConst(Constant::Boolean(false)).to_bytes(),
        vec![LBF]
    );
}

#[test]
fn serialize_load_const_null() {
    assert_eq!(Instruction::LoadConst(Constant::Null).to_bytes(), vec![LDN]);
}

#[test]
fn serialize_load_const_string() {
    let mut expected = vec![LDS];
    expected.extend_from_slice(&5u32.to_le_bytes());
    expected.extend_from_slice(b"hello");

    assert_eq!(
        Instruction::LoadConst(Constant::String("hello".into())).to_bytes(),
        expected
    );
}

#[test]
fn serialize_load_var() {
    let mut expected = vec![LDV];
    expected.extend_from_slice(&7u64.to_le_bytes());

    assert_eq!(Instruction::LoadVar(7).to_bytes(), expected);
}

#[test]
fn serialize_call() {
    assert_eq!(Instruction::Call.to_bytes(), vec![CALL]);
}

#[test]
fn serialize_call_native() {
    let mut expected = vec![NCALL];
    expected.extend_from_slice(&1u32.to_le_bytes());
    expected.extend_from_slice(&2u32.to_le_bytes());

    assert_eq!(Instruction::CallNative(1, 2).to_bytes(), expected);
}

#[test]
fn serialize_load_capture() {
    let mut expected = vec![LDCP];
    expected.extend_from_slice(&9u64.to_le_bytes());

    assert_eq!(Instruction::LoadCapture(9).to_bytes(), expected);
}

#[test]
fn serialize_load_function() {
    let mut expected = vec![LDFN];
    expected.extend_from_slice(&3u32.to_le_bytes());

    assert_eq!(Instruction::LoadFunction(3).to_bytes(), expected);
}

#[test]
fn serialize_load_collection() {
    let mut expected = vec![LDC];
    expected.extend_from_slice(&8u32.to_le_bytes());

    assert_eq!(Instruction::LoadCollection(8).to_bytes(), expected);
}

#[test]
fn serialize_load_from_collection() {
    assert_eq!(Instruction::LoadFromCollection.to_bytes(), vec![LDFC]);
}

#[test]
fn serialize_collection_len() {
    assert_eq!(Instruction::CollectionLen.to_bytes(), vec![LEN]);
}
