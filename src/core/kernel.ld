ENTRY(boot)

MEMORY {
    VECTOR (rx) : ORIGIN = 0x0, LENGTH = 0xFF
    FLASH (rx) : ORIGIN = 0x8000000, LENGTH = 1M
    SRAM (rw) : ORIGIN = 0x30000000, LENGTH = 8M
}

SECTIONS {
    .boot_stack (NOLOAD) : ALIGN(8) {
        _stack_end = .;
        . += 8K;
        _stack_start = .;
    } >SRAM

    .data : { *(.data*) } >SRAM

    .bss (NOLOAD) : ALIGN(16) {
        _bss_end = .;
        *(.bss*);
        . = ALIGN(16);
        _bss_start = .;
    } >SRAM

    .vt.boot : ALIGN(4) {
        KEEP(*(.vector.flash))
    } >FLASH

    .vt.root : ALIGN(4) {
        KEEP(*(.vector.root))
    } >VECTOR

    .text : {
        KEEP(*(.text))
        *(.text*)
        . = ALIGN(8);
    } >FLASH

    .rodata : {
        *(.rodata)
    } >FLASH

    /DISCARD/ : {
        *(.comment)
        *(.ARM.*)
    }
}
