#!/bin/bash

NAME=loop_loop

(
    cd wasm || exit
    wat2wasm --enable-threads ./$NAME.wat
)

(
    cd other || exit
    clang -O3 $NAME.c -o $NAME.out
)

cargo build

(
    cd other || exit

    printf "c: "
    ./$NAME.out

    printf "js: "
    deno run -A ./$NAME.ts

    printf "wasm-js: "
    deno run -A ./$NAME.wasm.ts

)

cargo build

printf "wasmer-singlepass: "
WASMER_COMPILER=singlepass cargo run --quiet ./wasm/$NAME.wat

printf "wasmer-cranelift: "
WASMER_COMPILER=cranelift cargo run --quiet ./wasm/$NAME.wat

printf "wasmer-llvm: "
WASMER_COMPILER=llvm cargo run --quiet ./wasm/$NAME.wat
