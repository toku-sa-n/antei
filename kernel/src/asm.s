// vim: set filetype=asm

.text
.code64
.intel_syntax noprefix

.macro  generic_handler vector fxsave_offset
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
	// After an interrupt or exception, if the exception pushes an error code,
	// `rsp mod 16` is 0. If the interrupt or exception does not push an error
	// code, `rsp mod 16` is 8, so we add `8` here.  See:
	// https://forum.osdev.org/viewtopic.php?f=1&t=22014
	sub rsp, 512+\fxsave_offset

	fxsave [rsp]

	call interrupt_handler_\vector

	fxrstor [rsp]

	add rsp, 512+\fxsave_offset

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

	.macro handler vector
	generic_handler \vector 8
	.endm

	.macro handler_with_error_code vector
	generic_handler \vector 0
	.endm

	handler_with_error_code 0x0e
	handler 0x20

	.global asm_switch_context

asm_switch_context:
	mov [rdi+0x00], rax
	mov [rdi+0x08], rbx
	mov [rdi+0x10], rcx
	mov [rdi+0x18], rdx

	lea rax, [rsp+0x08]
	mov [rdi+0x20], rax
	mov [rdi+0x28], rbp
	mov [rdi+0x30], rsi
	mov [rdi+0x38], rdi

	mov [rdi+0x40], r8
	mov [rdi+0x48], r9
	mov [rdi+0x50], r10
	mov [rdi+0x58], r11

	mov [rdi+0x60], r12
	mov [rdi+0x68], r13
	mov [rdi+0x70], r14
	mov [rdi+0x78], r15

	mov [rdi+0x80], cs
	mov [rdi+0x88], ss
	mov [rdi+0x90], fs
	mov [rdi+0x98], gs

	mov rax, cr3
	mov [rdi+0xa0], rax
	mov rax, [rsp]
	mov [rdi+0xa8], rax
	pushfq
	pop qword ptr [rdi+0xb0]

	fxsave [rdi+0xc0]

	mov rax, [rsi+0x00]
	mov rbx, [rsi+0x08]
	mov rcx, [rsi+0x10]
	mov rdx, [rsi+0x18]

	mov rbp, [rsi+0x28]
	mov rdi, [rsi+0x38]

	mov r8, [rsi+0x40]
	mov r9, [rsi+0x48]
	mov r10, [rsi+0x50]
	mov r11, [rsi+0x58]

	mov r12, [rsi+0x60]
	mov r13, [rsi+0x68]
	mov r14, [rsi+0x70]
	mov r15, [rsi+0x78]

	mov rax, [rsi+0x90]
	mov fs, ax
	mov rax, [rsi+0x98]
	mov gs, ax

	mov rax, [rsi+0xa0]
	mov cr3, rax

	fxrstor [rsi+0xc0]

	push qword ptr [rsi+0x88]
	push qword ptr [rsi+0x20]
	push qword ptr [rsi+0xb0]
	push qword ptr [rsi+0x80]
	push qword ptr [rsi+0xa8]

	mov rsi, [rsi+0x30]

	iretq
