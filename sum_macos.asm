; Example from https://www.jdoodle.com/compile-assembler-nasm-online/

    global  _main
    default rel

section .text

_main:

    mov     ax, [x]
    sub     ax, '0'
    mov     bx, [y]
    sub     bx, '0'
    add     ax, bx
    add     ax, '0'

    mov     ah, 10      ; newline
    mov     [sum], ax

    mov     rsi, msg
    mov     rdx, len
    mov     rdi, 1
    mov     rax, 0x02000004
    syscall

    mov     rsi, sum
    mov     rdx, 2
    mov     rdi, 1
    mov     rax, 0x02000004
    syscall

    mov     rax, 0x02000001
    xor     rdi, rdi
    syscall

section .data
    x: db '5'
    y: db '3'
    msg: db  "sum of x and y is "
    len equ $ - msg

segment .bss

    sum: resb 2
