elf_files = bootloader.o print.o readDisk.o secondStage.o gdt_64.o CPUID.o paging.o binaries.o
linker_file = linker.ld


compile: $(elf_files) $(linker_file)
	cd kernel; cargo +nightly build --release
	ld --script $(linker_file)
	objcopy -O binary --gap-fill 0x00 --pad-to 0x20000 bootloader.elf bootloader.bin


$(elf_files): %.o: %.asm
	rm rootfile.tar
	tar -cvf rootfile.tar root/
	nasm -f elf64 $< -o $@

run:
	qemu-system-x86_64 -hda bootloader.bin -serial stdio
.PHONY: clean binaries.o

clean:
	rm $(elf_files) bootloader.elf
	cd kernel; cargo clean


