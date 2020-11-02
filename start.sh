nasm -f elf64 bootloader.asm -o bootloader.o
nasm -f elf64 print.asm -o print.o
nasm -f elf64 readDisk.asm -o readDisk.o
ld --script linker.ld
objcopy -O binary bootloader.elf bootloader.bin
rm bootloader.o print.o readDisk.o bootloader.elf
qemu-system-x86_64 -fda bootloader.bin
