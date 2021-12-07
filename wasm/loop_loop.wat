(module
  (import "env" "memory" (memory $mem 1 1024 shared))
  (data (i32.const 16) "abcd")
    (func (export "fill_0")
    (local $i i32)
    (local $j i32)

    (set_local $i (i32.const 0))
    (block $first_brk
      (loop $first_loop

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
