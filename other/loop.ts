// deno-lint-ignore-file camelcase

async function compile_wat(wat: string) {
    Deno.writeTextFileSync("./.tmp.wat", wat);
    await Deno.run({cmd: "wat2wasm ./.tmp.wat".split(" ")}).status();
    const code = Deno.readFileSync("./.tmp.wasm");
    Deno.remove("./.tmp.wat");
    Deno.remove("./.tmp.wasm");
    const wasm = await WebAssembly.compile(code);
    const inst = new WebAssembly.Instance(wasm, {});
    return inst;
}

const LIM = 1000000;

const inst = await compile_wat(`
  (module
    (memory (export "memory") 1)
    (data (i32.const 16) "abcd")
      (func (export "fill_0")
      (local $i i32)
      (local $j i32)

      (set_local $i (i32.const 0))
      (block $first_brk
        (loop $first_loop
          ${/*(
            function() {
              let str = "";
              for (let i = 0; i < 16384; ++i) {
                str += "i32.const "+(i*4)+"\n";
                str += "i32.const "+i+"\n";
                str += "i32.store\n";
              }
              return str;
            }
          )()*/""}

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
`);

const _memory = new Uint32Array((inst.exports.memory as any).buffer);
const array = new Uint32Array(16384);

function time(fn: ()=>void) {
  const start = Date.now();
  const result = fn();
  console.log("time: " + ((Date.now() - start) / 1000));
  return result;
}

time(() => {
  (inst.exports.fill_0 as any)();
});

// time(() => {
//   for (let j = 0; j < LIM; ++j) {
//     for (let i = 0; i < 16384; ++i) {
//       array[i] = i;
//     }
//   }
// });
