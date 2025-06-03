OUTPUT_FORMAT(elf64-x86-64)

ENTRY(_start)

BOOT_OFFSET = 0x100000;
KERNEL_OFFSET = 0x100400;
/* KERNEL_OFFSET = 0xFFFFFF0000000000; */

PHDRS {
    multiboot2 PT_LOAD FLAGS(5);
}

SECTIONS {
    . = BOOT_OFFSET;

    /* ensure that the multiboot header is at the beginning */
    .multiboot2 ALIGN(4K) : AT(BOOT_OFFSET) {
        KEEP(*(.multiboot2))
        . = ALIGN(8);
    }

    . = KERNEL_OFFSET;
    .dummy ALIGN(4K) : AT(ADDR(.dummy) - KERNEL_OFFSET) {}

    .text ALIGN(4K) : AT(ADDR(.text) - KERNEL_OFFSET) {
        *(.text*)
    }

    .rodata ALIGN(4K) : AT(ADDR(.rodata) - KERNEL_OFFSET) {
        *(.rodata*)
    }

    .data ALIGN(4K) : AT(ADDR(.data) - KERNEL_OFFSET) {
        *(.data*)
    }

    .bss ALIGN(4K) : AT(ADDR(.bss) - KERNEL_OFFSET) {
        *(COMMON)
        *(.bss*)
    }

    __end = .;

    /DISCARD/ : {
        *(.comment*)
        *(.eh_frame*)
        *(.gcc_except_table*)
        *(.note*)
        *(.rel.eh_frame*)
    }
}
