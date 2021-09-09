// vim: set filetype=asm

.text
.code64
.intel_syntax noprefix

.global execute_syscall

execute_syscall:
	// fn execute_syscall(index: u64, a1: u64, a2: u64);
	// index: rdi
	// a1: rsi
	// a2: rdx

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
	// When a function is called, `rsp mod 16 == 8`, so we add `8` here.
	sub    rsp, 512+8
	fxsave [rsp]

	syscall

	fxrstor [rsp]
	add     rsp, 512+8
	pop     r11
	pop     r10
	pop     r9
	pop     r8
	pop     rdi
	pop     rsi
	pop     rdx
	pop     rcx
	pop     rax

	mov rsp, rbp
	pop rbp

	ret
