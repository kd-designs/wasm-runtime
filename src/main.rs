use rust_wasm_runtime::{Instruction, Value, VM};

fn main() {
    println!("=== WebAssembly Stack VM Demonstration ===");

    let mut vm = VM::new(1); // 1 page (64KiB)

    // Function: add_and_multiply(a, b, c) -> (a + b) * c
    let instructions = vec![
        Instruction::LocalGet(0),
        Instruction::LocalGet(1),
        Instruction::I32Add,
        Instruction::LocalGet(2),
        Instruction::I32Mul,
        Instruction::Return,
    ];

    let mut locals = vec![
        Value::I32(5),  // a = 5
        Value::I32(15), // b = 15
        Value::I32(3),  // c = 3
    ];

    println!("Inputs: a = 5, b = 15, c = 3");
    println!("Executing: (a + b) * c ...");

    match vm.execute(&instructions, &mut locals) {
        Ok(Some(Value::I32(result))) => {
            println!("Execution Result: {}", result);
            assert_eq!(result, (5 + 15) * 3);
            println!("Verification Passed!");
        }
        res => {
            println!("Unexpected execution result: {:?}", res);
        }
    }
}
