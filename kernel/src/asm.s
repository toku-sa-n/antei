.text
.code64
.intel_syntax noprefix

.macro  handler vector
.extern interrupt_handler_\vector
.global asm_interrupt_handler_\vector

asm_interrupt_handler_\vector:
	push rbp
	mov  rbp, rsp

	push rax
	push rcx
	push rdx
	push rsi
	push rdi
	push r8
	push r9
	push r10
	push r11

	// `fxsave` saves 512-byte data, and it requires a 16-byte aligned address.
	// After the `call` instruction, `rsp mod 16` is 8, so we add `8` here.
	sub rsp, 512+8

	fxsave [rsp]

	call interrupt_handler_\vector

	fxrstor [rsp]

	add rsp, 512+8

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
