; ----------------------------------------------------------------------------------------
; Writes "Hello, World" to the console using only system calls. Runs on 64-bit macOS only.
; To assemble and run:
;
;     nasm -fmacho64 hello.asm && ld hello.o && ./a.out
; ----------------------------------------------------------------------------------------

          global    _main
          default   rel

%assign   LOOP_COUNT 5

          section   .text
_main:    mov       rax, 0x02000004         ; system call for write
          mov       rdi, 1                  ; file handle 1 is stdout
; Old style absolute addressing
;          mov       rsi, message            ; address of string to output
; New style relative addressing          
          lea       rsi, [message]          ; address of string to output
          mov       rdx, message.len        ; number of bytes
          syscall                           ; invoke operating system to do the write

          mov       rbx, 0
loop1:
          call      hello
          inc       rbx
          cmp       rbx, LOOP_COUNT
          jl       loop1

          mov       rax, 0x02000004         ; system call for write
          mov       rdi, 1                  ; file handle 1 is stdout
          lea       rsi, [var2]             ; address of string to output
          mov       rdx, var2Len            ; number of bytes
          syscall                           ; invoke operating system to do the write

          mov       rax, 0x02000001         ; system call for exit
          xor       rdi, rdi                ; exit code 0
          syscall                           ; invoke operating system to exit

; Hello world function
hello:
          mov       rax, 0x02000004         ; system call for write
          mov       rdi, 1                  ; file handle 1 is stdout
          lea       rsi, [var1]             ; address of string to output
          mov       rdx, var1Len            ; number of bytes
          syscall                           ; invoke operating system to do the write
          ret

          section   .data

message:  db        "Hello, World", 10      ; note the newline at the end
.len      equ       $ - message

var1:       db      "123", 10
var1Len     equ     $ - var1

var2:       TIMES 5 db      "987€ "
            db      10
var2Len     equ     $ - var2
