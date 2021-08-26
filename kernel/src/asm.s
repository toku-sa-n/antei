    .text
    .code64
    .intel_syntax noprefix


    .macro pushxmm

    sub rsp, 16*16
    movdqu [rsp+16*0], xmm0
    movdqu [rsp+16*1], xmm1
    movdqu [rsp+16*2], xmm2
    movdqu [rsp+16*3], xmm3
    movdqu [rsp+16*4], xmm4
    movdqu [rsp+16*5], xmm5
    movdqu [rsp+16*6], xmm6
    movdqu [rsp+16*7], xmm7
    movdqu [rsp+16*8], xmm8
    movdqu [rsp+16*9], xmm9
    movdqu [rsp+16*10], xmm10
    movdqu [rsp+16*11], xmm11
    movdqu [rsp+16*12], xmm12
    movdqu [rsp+16*13], xmm13
    movdqu [rsp+16*14], xmm14
    movdqu [rsp+16*15], xmm15

    .endm

    .macro popxmm

    movdqu xmm15, [rsp+16*15]
    movdqu xmm14, [rsp+16*14]
    movdqu xmm13, [rsp+16*13]
    movdqu xmm12, [rsp+16*12]
    movdqu xmm11, [rsp+16*11]
    movdqu xmm10, [rsp+16*10]
    movdqu xmm9, [rsp+16*9]
    movdqu xmm8, [rsp+16*8]
    movdqu xmm7, [rsp+16*7]
    movdqu xmm6, [rsp+16*6]
    movdqu xmm5, [rsp+16*5]
    movdqu xmm4, [rsp+16*4]
    movdqu xmm3, [rsp+16*3]
    movdqu xmm2, [rsp+16*2]
    movdqu xmm1, [rsp+16*1]
    movdqu xmm0, [rsp+16*0]
    add rsp, 16*16

    .endm

    .macro handler vector
    .extern interrupt_handler_\vector
    .global asm_interrupt_handler_\vector
asm_interrupt_handler_\vector:
    push rbp
    mov rbp, rsp

    push rax
    push rcx
    push rdx
    push rsi
    push rdi
    push r8
    push r9
    push r10
    push r11

    pushxmm

    call interrupt_handler_\vector

    popxmm

    pop r11
    pop r10
    pop r9
    pop r8
    pop rdi
    pop rsi
    pop rdx
    pop rcx
    pop rax

    mov rsp, rbp
    pop rbp

    iretq
.endm

    handler 0x0e
    handler 0x20
