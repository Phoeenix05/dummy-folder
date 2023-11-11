.global _start
.p2align 2

_start: mov w12, 0xff
        cmp x0, #2
        b.ne exit

        ldr x11, [x1, #8]       ; get pointer at x1 + 8
        ldrb w11, [x11]         ; load the byte pointed to by that pointer into w11

        ; FIXME: idk what I should do to print argv[1]
        ; mov x0, #1              ; set x0 to stdout
        ; mov x1, x2
        ; ldr x3, [x11, #8]
        ; mov x8, #64
        ; svc #0x80               ; syscall to print to stdout

        mov w12, 0x0            ; hopefully sets the exit code to 0

exit:   mov w0, w12     ; move return code into x0 so it can be queried at OS level
        mov x16, #1     ; syscall n1 terminated the program
        svc #0x80       ; call darwin (kernel) to terminate the program

; msg: .asciz "Hallo!"
