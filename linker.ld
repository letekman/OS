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

    .secondStage 0x8000  : {
        *(.text);
        *(.data);
        librust_os.a(*);
        FILL(0x00);
        . += 0x3E00;
    }

    
}
