#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValType {
    I32 = 0x7F,
    I64 = 0x7E,
    F32 = 0x7D,
    F64 = 0x7C,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

#[derive(Debug, Clone)]
pub struct FuncType {
    pub params: Vec<ValType>,
    pub returns: Vec<ValType>,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    I32Const(i32),
    I32Add,
    I32Sub,
    I32Mul,
    LocalGet(u32),
    LocalSet(u32),
    Return,
    End,
}
