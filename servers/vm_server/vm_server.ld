OUTPUT_FORMAT(elf64-x86-64);

ENTRY(main);

SECTIONS {
    . = 0x1000;

    .text : {
        *(.text*)
    }

    .rodata : {
        *(.rodata*)
    }

    .data : {
        *(.data)
    }

    .bss : {
        *(.bss)
    }

    .eh_frame : {
        *(.eh_frame)
    }

    /DISCARD/ : {
        *(.init)
        *(.fini)
    }
}
