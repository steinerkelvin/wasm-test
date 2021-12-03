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

use wasmer::{imports, wat2wasm, Instance, Memory, MemoryType, Module, Store, Value};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_compiler_singlepass::Singlepass;
use wasmer_engine_universal::Universal;
use std::time::{Duration, Instant};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Let's declare the Wasm module with the text representation.
    let wasm_bytes = wat2wasm(
//         r#"
// (module
//   (type $sum_t (func (param i32 i32) (result i32)))
//   (func $sum_f (type $sum_t) (param $x i32) (param $y i32) (result i32)
//     local.get $x
//     local.get $y
//     i32.add)
//   (export "sum" (func $sum_f)))
// "#
        r#"
  (module
    (import "env" "memory" (memory $mem 1))
    (data (i32.const 16) "abcd")
      (func (export "fill_0")
      (local $i i32)
      (local $j i32)

      (set_local $i (i32.const 0))
      (block $first_brk
        (loop $first_loop

          ;; ${(
          ;;   function() {
          ;;     let str = "";
          ;;     for (let i = 0; i < 16384; ++i) {
          ;;       str += "i32.const "+(i*4)+"\n";
          ;;       str += "i32.const "+i+"\n";
          ;;       str += "i32.store\n";
          ;;     }
          ;;     return str;
          ;;   }
          ;; )()}

          (set_local $j (i32.const 0))
          (block $second_brk
          (loop $second_loop
            (get_local $j)
            (i32.const 4)
            (get_local $j)
            (i32.mul)
            (i32.store)

            (set_local $j (i32.add (get_local $j) (i32.const 1)))
            (br_if $second_brk (i32.eq (get_local $j) (i32.const 16384)))
            (br $second_loop)
          )
          )

          (set_local $i (i32.add (get_local $i) (i32.const 1)))
          (br_if $first_brk (i32.eq (get_local $i) (i32.const 1000000)))
          (br $first_loop)
        )
      )
    )
  )

        "#
        .as_bytes(),
    )?;

    // Define a compiler configuration.
    //
    // In this situation, the compiler is
    // `wasmer_compiler_cranelift`. The compiler is responsible to
    // compile the Wasm module into executable code.
    let compiler = Cranelift::default();
    // Use Singlepass compiler with the default settings
    //let compiler = Singlepass::default();

    println!("Creating Universal engine...");
    // Define the engine that will drive everything.
    //
    // In this case, the engine is `wasmer_engine_universal` which roughly
    // means that the executable code will live in memory.
    let engine = Universal::new(compiler).engine();

    // Create a store, that holds the engine.
    let store = Store::new(&engine);

    println!("Compiling module...");
    // Here we go.
    //
    // Let's compile the Wasm module. It is at this step that the Wasm
    // text is transformed into Wasm bytes (if necessary), and then
    // compiled to executable code by the compiler, which is then
    // stored in memory by the engine.
    let module = Module::new(&store, wasm_bytes)?;

    let mem = Memory::new(&store, MemoryType::new(1, None, false)).unwrap();

    // Create an import object. Since our Wasm module didn't declare
    // any imports, it's an empty object.
    let import_object = imports! {
        "env" => {
            "memory" => mem,
        }
    };

    println!("Instantiating module...");
    // And here we go again. Let's instantiate the Wasm module.
    let instance = Instance::new(&module, &import_object)?;

    println!("Calling function...");
    let func = instance.exports.get_function("fill_0")?;
    
    // let results = sum.call(&[Value::I32(1), Value::I32(2)])?;
    // assert_eq!(results.to_vec(), vec![Value::I32(3)]);

    let start = Instant::now();
    let results = func.call(&[])?;
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);

    assert_eq!(results.to_vec(), vec![]);

    println!("Results: {:?}", results);

    Ok(())
}

#[test]
fn test_engine_universal() -> Result<(), Box<dyn std::error::Error>> {
    main()
}
