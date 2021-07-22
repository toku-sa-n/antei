OUTPUT_FORMAT(elf64-x86-64);

ENTRY(main);

MEMORY {
    kernel (WX) : ORIGIN = 0xffffffff80000000, LENGTH = 0x20000000
}

SECTIONS {
    . = 0xffffffff80000000;

    .text : {
        *(.text*)
    } > kernel

    .rodata : {
        *(.rodata*)
    } > kernel

    .data : {
        *(.data)
    } > kernel

    .bss : {
        *(.bss)
    } > kernel

    .eh_frame : {
        *(.eh_frame)
    } > kernel

    /DISCARD/ : {
        *(.init)
        *(.fini)
    }
}
