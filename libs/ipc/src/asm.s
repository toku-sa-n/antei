.text
.code64
.intel_syntax noprefix

.global asm_syscall

asm_syscall:
	push rax
	push rcx
	push rdx
	push rsi
	push rdi
	push r8
	push r9
	push r10
	push r11

	// 8 for the alignment. `fxsave` requires 16-byte aligned address, and after the `call` instruction `rsp % 16 == 8`.
	sub    rsp, 512+8
	fxsave [rsp]

	mov rax, rdi
	mov rdi, rsi
	mov rsi, rdx

	syscall

	fxrstor [rsp]
	add     rsp, 512+8

	pop r11
	pop r10
	pop r9
	pop r8
	pop rdi
	pop rsi
	pop rdx
	pop rcx
	pop rax

	ret
