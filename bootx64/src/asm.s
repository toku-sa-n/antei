    .text
    .code64
    .intel_syntax noprefix

    # RDI: A pointer to boot information.
    # RSI: Entry address.
    # RDX: New stack pointer.
    .global switch_stack_and_call_kernel_code
switch_stack_and_call_kernel_code:
    mov rsp, rdx
    jmp rsi
