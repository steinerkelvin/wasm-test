// deno-lint-ignore-file camelcase

function time(fn: () => void) {
  const start = Date.now();
  const result = fn();
  console.log((Date.now() - start) / 1000);
  return result;
}

const LIM = 1000000;
const array = new Uint32Array(16384);
time(() => {
  for (let j = 0; j < LIM; ++j) {
    for (let i = 0; i < 16384; ++i) {
      array[i] = i;
    }
  }
});
