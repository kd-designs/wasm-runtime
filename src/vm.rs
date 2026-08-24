use crate::types::{Instruction, Value};

pub struct VM {
    pub stack: Vec<Value>,
    pub memory: Vec<u8>,
}

impl VM {
    pub fn new(memory_pages: usize) -> Self {
        Self {
            stack: Vec::with_capacity(1024),
            memory: vec![0u8; memory_pages * 64 * 1024],
        }
    }

    pub fn execute(&mut self, instructions: &[Instruction], locals: &mut [Value]) -> Result<Option<Value>, &'static str> {
        let mut pc = 0;

        while pc < instructions.len() {
            match &instructions[pc] {
                Instruction::I32Const(val) => {
                    self.stack.push(Value::I32(*val));
                }
                Instruction::I32Add => {
                    let b = self.pop_i32()?;
                    let a = self.pop_i32()?;
                    self.stack.push(Value::I32(a.wrapping_add(b)));
                }
                Instruction::I32Sub => {
                    let b = self.pop_i32()?;
                    let a = self.pop_i32()?;
                    self.stack.push(Value::I32(a.wrapping_sub(b)));
                }
                Instruction::I32Mul => {
                    let b = self.pop_i32()?;
                    let a = self.pop_i32()?;
                    self.stack.push(Value::I32(a.wrapping_mul(b)));
                }
                Instruction::LocalGet(idx) => {
                    let val = locals.get(*idx as usize).ok_or("Local index out of bounds")?.clone();
                    self.stack.push(val);
                }
                Instruction::LocalSet(idx) => {
                    let val = self.stack.pop().ok_or("Stack underflow on local.set")?;
                    let target = locals.get_mut(*idx as usize).ok_or("Local index out of bounds")?;
                    *target = val;
                }
                Instruction::Return => {
                    break;
                }
                Instruction::End => {
                    break;
                }
            }
            pc += 1;
        }

        Ok(self.stack.pop())
    }

    fn pop_i32(&mut self) -> Result<i32, &'static str> {
        match self.stack.pop() {
            Some(Value::I32(v)) => Ok(v),
            _ => Err("Expected I32 value on top of stack"),
        }
    }
}
