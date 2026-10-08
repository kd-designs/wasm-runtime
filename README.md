# rust-wasm-runtime

A WebAssembly MVP bytecode parser, LEB128 decoder, and stack virtual machine interpreter in Rust.

## Features

- **LEB128 Decoder**: Variable-length signed/unsigned 32-bit and 64-bit integer parser.
- **WASM Stack Machine**: Value stack supporting 32/64-bit integer/float operations.
- **Linear Memory**: Page-allocated memory buffer (64KiB per page).
- **Arithmetic & Control Flow**: `i32.const`, `i32.add`, `i32.sub`, `i32.mul`, `local.get`, `local.set`, and `return`.
