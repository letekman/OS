compile: bootloader.o print.o readDisk.o secondStage.o gdt_64.o CPUID.o paging.o linker.ld
	ld --script linker.ld
	objcopy -O binary bootloader.elf bootloader.bin

bootloader.o: bootloader.asm
	nasm -f elf64 $< -o $@

print.o: print.asm
	nasm -f elf64 $< -o $@

readDisk.o: readDisk.asm
	nasm -f elf64 $< -o $@

secondStage.o: secondStage.asm
	nasm -f elf64 $< -o $@

gdt_64.o: gdt_64.asm
	nasm -f elf64 $< -o $@

CPUID.o: CPUID.asm
	nasm -f elf64 $< -o $@

paging.o: paging.asm
	nasm -f elf64 $< -o $@

run:
	make compile
	qemu-system-x86_64 -hda bootloader.bin 

debug:
	qemu-system-x86_64 -s -S -hda bootloader.bin & gdb


.PHONY: clean

clean:
	rm bootloader.o print.o readDisk.o paging.o CPUID.o gdt_64.o secondStage.o bootloader.elf

