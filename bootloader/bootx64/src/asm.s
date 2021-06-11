    .text
    .code64
    .intel_syntax noprefix

    .global asm_disable_page_table_write_protect
asm_disable_page_table_write_protect:
    mov rax, cr0
    and eax, 0xfffeffff
    mov cr0, rax
    ret
