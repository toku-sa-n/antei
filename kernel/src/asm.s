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

	.global switch_context

switch_context:
	// fn switch_context(old_cx: *mut Context, new_cx: *mut Context);
	// old_cx: RDI
	// new_cx: RSI

	mov [rdi+0x00], rax
	mov [rdi+0x08], rbx
	mov [rdi+0x10], rcx
	mov [rdi+0x18], rdx

	lea rax, [rsp+0x08]
	mov [rdi+0x20], rax
	mov [rdi+0x28], rbp
	mov [rdi+0x30], r8
	mov [rdi+0x38], r9
	mov [rdi+0x40], r10
	mov [rdi+0x48], r11
	mov [rdi+0x50], r12
	mov [rdi+0x58], r13
	mov [rdi+0x60], r14
	mov [rdi+0x68], r15

	mov rax, cr3
	mov [rdi+0x70], rax
	mov [rdi+0x78], cs
	mov [rdi+0x80], ss
	mov [rdi+0x88], fs
	mov [rdi+0x90], gs

	mov rax, [rsp]
	mov [rdi+0x98], rax
	pushfq
	pop qword [rdi+0x100]

	fxsave [rdi+0x110]

	mov rax, [rsi+0x70]
	mov cr3, rax

	mov rax, [rsi+0x88]
	mov fs, ax

	mov rax, [rsi+0x90]
	mov gs, ax

	mov rax, [rsi+0x00]
	mov rbx, [rsi+0x08]
	mov rcx, [rsi+0x10]
	mov rdx, [rsi+0x18]
	mov rbp, [rsi+0x28]
	mov r8, [rsi+0x30]
	mov r9, [rsi+0x38]
	mov r10, [rsi+0x40]
	mov r11, [rsi+0x48]
	mov r12, [rsi+0x50]
	mov r13, [rsi+0x58]
	mov r14, [rsi+0x60]
	mov r15, [rsi+0x68]

	fxrstor [rsi+0x110]

	pushq [rsi+0x80]
	pushq [rsi+0x20]
	pushq [rsi+0x100]
	pushq [rsi+0x78]
	pushq [rsi+0x98]

	iretq
