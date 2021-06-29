    .section .rodata
    .code64
    .intel_syntax noprefix

    .global init_binary
init_binary:
    .incbin "../target/debug/init"
