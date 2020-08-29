section .bss
    var1: resb 20

section .text
	global _main
	default rel

_main:
    mov eax, 73651

int_to_string:
    xor ebx, ebx ; digit counter
    mov ecx, 10 ; divisor
.push_chars:
    xor edx, edx
    div ecx
    add edx, '0'
    push rdx ; push edx did not work on the online nasm compiler
    inc ebx
    test eax, eax
    jnz .push_chars

    lea rdi, [var1] ; store digits here
    mov edx, ebx ; save string length
    inc edx ; add 1 for \n

.pop_chars:
    pop rax
    stosb
    dec ebx
    test ebx, ebx
    jnz .pop_chars
    mov eax, 10 ; \n
    stosb

          mov       rax, 0x02000004         ; system call for write
          mov       rdi, 1                  ; file handle 1 is stdout
          lea       rsi, [var1]             ; address of string to output
          ; number of bytes is already in RDX
          syscall                           ; invoke operating system to do the write

          mov       rax, 0x02000001         ; system call for exit
          xor       rdi, rdi                ; exit code 0
          syscall                           ; invoke operating system to exit
