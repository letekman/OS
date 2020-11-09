
pub unsafe fn inb(port: u16) -> u8 {
    let retval: u8;
    llvm_asm!("inb %dx, %al" : "={al}"(retval) : "{dx}"(port) :: "volatile");
    retval
}
pub unsafe fn outb(value: u8, port: u16) {
    llvm_asm!("outb %al, %dx" :: "{dx}"(port), "{al}"(value) :: "volatile");
}
pub unsafe fn inw(port: u16) -> u16 {
    let retval: u16;
    llvm_asm!("inw %dx, %ax" : "={ax}"(retval) : "{dx}"(port) :: "volatile");
    retval
}
pub unsafe fn outw(value: u16, port: u16) {
    llvm_asm!("outw %ax, %dx" :: "{dx}"(port), "{ax}"(value) :: "volatile");
}
pub unsafe fn inl(port: u16) -> u32 {
    let retval: u32;
    llvm_asm!("inl %dx, %eax" : "={eax}"(retval) : "{dx}"(port) :: "volatile");
    retval
}
pub unsafe fn outl(value: u32, port: u16) {
    llvm_asm!("outl %eax, %dx" :: "{dx}"(port), "{eax}"(value) :: "volatile");
}
pub unsafe fn io_wait() {
    llvm_asm!("jmp 1f;1:jmp 2f;2:" :::: "volatile");
}