nasm -f elf64 bootloader.asm -o bootloader.o
nasm -f elf64 print.asm -o print.o
nasm -f elf64 readDisk.asm -o readDisk.o
nasm -f elf64 secondStage.asm -o secondStage.o
nasm -f elf64 gdt_64.asm -o gdt_64.o
nasm -f elf64 CPUID.asm -o CPUID.o
nasm -f elf64 paging.asm -o paging.o

ld --script linker.ld
objcopy -O binary bootloader.elf bootloader.bin
rm bootloader.o print.o readDisk.o paging.o CPUID.o gdt_64.o secondStage.o
qemu-system-x86_64 -s -S -hda bootloader.bin  -d cpu -D log.txt
