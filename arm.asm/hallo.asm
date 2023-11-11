.global _start
.p2align 2

; _start: mov     X0, #1
;         adr     X1, helloworld
;         mov     X2, #13
;         mov     X16, #4
;         svc     #0x80

;         mov     X0, #0
;         mov     X16, #1
;         svc     #0x80

; helloworld: .ascii "Hallo World!\n"

_start: mov w12, 0xff
        cmp x0, #2
        bne exit

        ldr x11, [x1, #8]       ; get pointer at x1 + 8
        ldrb w11, [x11]         ; load the byte pointed to by that pointer into w11

exit:   mov w0, w12     ; move return code into x0 so it can be queried at OS level
        mov x16, #1     ; syscall n1 terminated the program
        svc #0x80       ; call darwin (kernel) to terminate the program
