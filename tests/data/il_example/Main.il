(map
    (width 20)
    (height 15)
    (event 
        (x (const 1)) 
        (y (range 1 2)) 
        (pages 
            ((command $test))
        )
    )
    (command $test (
        (if (variable.eq (single 2) (const 3))
            (then
                (switch.enable (single 1))
                (switch.disable (range 1 2))
                (switch.toggle (pointer 3))

                (variable.set (single 1) (const -1))
                (variable.mod (range 1 2) (random 0 512))
                (variable.div (pointer 3) (pointer 4))
            )
            (else
                (loop (
                    (wait 1.0)
                    (label $one)
                    (goto $three)
                    (label $two)
                    (goto $four)
                    (label $three)
                    (goto $two)
                    (label $four)
                    (wait 0.5)
                ))
                (variable.mul (pointer 5) (single 6))
            )
        )

        ;; this is a code comment (ignored)
        "this is an r2k comment (included)"
        (variable.set (range 1 2) (const 2))

        (loop (
            erase
            return
            (call.common 1)
            (call.direct 2 10)
            (call.indirect 3 4)
        ))
    ))
)
