//! Defining an engine in Wasmer is one of the fundamental steps.
//!
//! This example illustrates how to use the `wasmer_engine_universal`,
//! aka the Universal engine. An engine applies roughly 2 steps:
//!
//!   1. It compiles the Wasm module bytes to executable code, through
//!      the intervention of a compiler,
//!   2. It stores the executable code somewhere.
//!
//! In the particular context of the Universal engine, the executable
//! code is stored in memory.
//!
//! You can run the example directly by executing in Wasmer root:
//!
//! ```shell
//! cargo run --example engine-universal --release --features "cranelift"
//! ```
//!
//! Ready?

#![allow(unused_imports)]

use std::env;
use std::fs;
use std::time::Instant;

use wasmer::{imports, wat2wasm, Instance, Memory, MemoryType, Module, Store, Value, CompilerConfig, Features};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_compiler_singlepass::Singlepass;
use wasmer_compiler_llvm::LLVM;
use wasmer_engine_universal::Universal;

enum Compiler {
  Singlepass,
  Cranelift,
  LLVM,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    panic!("Missing first argument WAT_FILE.");
  }
  let wat_file_path = args[1].clone();

  let compiler_arg = env::var("WASMER_COMPILER").unwrap_or("cranelift".to_string());
  let compiler_chosen =
    match compiler_arg.to_lowercase().as_str() {
      "singlepass" => Compiler::Singlepass,
      "cranelift" => Compiler::Cranelift,
      "llvm" => Compiler::LLVM,
      _ => panic!("Unknown compiler: {}", compiler_arg),
    };

  let wat_text = fs::read_to_string(wat_file_path).unwrap();

  // Let's declare the Wasm module with the text representation.
  let wasm_bytes = wat2wasm(wat_text.as_bytes())?;

  // Define a compiler configuration.
  let compiler: Box<dyn CompilerConfig> = match compiler_chosen {
    Compiler::Singlepass => Box::new(Singlepass::default()),
    Compiler::Cranelift => Box::new(Cranelift::default()),
    Compiler::LLVM => Box::new(LLVM::default()),
  };

  let mut features = Features::new();
  features.threads(true);
  let features = features;

  let engine = Universal::new(compiler).features(features).engine();

  let store = Store::new(&engine);

  // Let's compile the Wasm module. It is at this step that the Wasm
  // text is transformed into Wasm bytes (if necessary), and then
  // compiled to executable code by the compiler, which is then
  // stored in memory by the engine.
  let module = Module::new(&store, wasm_bytes)?;

  let mem = Memory::new(&store, MemoryType::new(1, Some(1024), true)).unwrap();

  let import_object = imports! {
      "env" => {
          "memory" => mem,
      }
  };

  //println!("Instantiating module...");
  let instance = Instance::new(&module, &import_object)?;

  //println!("Calling function...");
  let func = instance.exports.get_function("fill_0")?;

  // let results = sum.call(&[Value::I32(1), Value::I32(2)])?;
  // assert_eq!(results.to_vec(), vec![Value::I32(3)]);

  let start = Instant::now();
  let results = func.call(&[])?;
  let duration = start.elapsed();

  println!("{:?}", duration);

  assert_eq!(results.to_vec(), vec![]);

  //println!("Results: {:?}", results);

  Ok(())
}

#[test]
fn test_engine_universal() -> Result<(), Box<dyn std::error::Error>> {
  main()
}
