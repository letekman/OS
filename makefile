elf_files = bootloader.o print.o readDisk.o secondStage.o gdt_64.o CPUID.o paging.o
linker_file = linker.ld
compile: $(elf_files) $(linker_file)
	ld --script $(linker_file)
	objcopy -O binary bootloader.elf bootloader.bin

$(elf_files): %.o: %.asm
	nasm -f elf64 $< -o $@

run:
	qemu-system-x86_64 -hda bootloader.bin 

.PHONY: clean

clean:
	rm $(elf_files) bootloader.elf

