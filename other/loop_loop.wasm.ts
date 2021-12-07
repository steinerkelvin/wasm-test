// deno-lint-ignore-file camelcase

function time(fn: () => void) {
  const start = Date.now();
  const result = fn();
  console.log((Date.now() - start) / 1000);
  return result;
}

const wasm_code = await Deno.readFile("./wasm/loop_loop.wasm");
const module = await WebAssembly.compile(wasm_code);

const memory = new WebAssembly.Memory({
  initial: 1,
  maximum: 16,
  shared: true,
});
const inst = new WebAssembly.Instance(module, { env: { memory } });
time(() => {
  (inst.exports.fill_0 as () => void)();
});
