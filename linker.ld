OUTPUT_FORMAT(elf64-x86-64)
SEARCH_DIR(kernel/target/target/release)
ENTRY(_bootstart)
OUTPUT(
	bootloader.elf
)
INPUT (
	bootloader.o
	print.o
    readDisk.o
    secondStage.o
    gdt_64.o
    paging.o
    CPUID.o
    binaries.o
    librust_os.a
)



SECTIONS
{
    .bootloader 0x7C00 : SUBALIGN(2) {
        bootloader.o(.text);
        print.o(.text);
        readDisk.o(.text);
    }
   
    .bootloader_data : SUBALIGN(2) {
        bootloader.o(.data);
        print.o(.data);
        readDisk.o(.data);

        bootloader.o(.rodata);
        print.o(.rodata);
        readDisk.o(.rodata);
       
    }

    /* Place the boot signature at LMA/VMA 0x7DFE */
    .sig 0x7DFE : {
        SHORT(0xaa55);
    } 

    .secondStage 0x8000 :
    {
        *(.text);
        *(.data);
        binaries.o(.rodata);
        librust_os.a(*);
        FILL(0xDEAD);
        . = 0x12000;
    }
    .idt BLOCK(0x1000) :
    {
        _IDT = .;
        . += 0x1000;
    }
    . = 0x29000;
    .stack BLOCK(0xF000) :
    {
        _STACK = 0x40000;
    }
    . = 0x60000;
    . = 0x100000;
    
    _HEAP_SIZE = 0x100000;
    .heap BLOCK(0x100000) :
    {
        _HEAP_START = .;
    }
    . = 0x200000;
}
