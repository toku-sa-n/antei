    .intel_syntax noprefix
    .code64
    .text

    .global asm_stop
asm_stop:
    mov rax, 0x55aa55aa55aa55aa
    jmp $
