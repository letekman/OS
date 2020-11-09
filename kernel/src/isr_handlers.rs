
use crate::io_ports;
use crate::PIC8259;

macro_rules! pushall {
    () => {{
        llvm_asm!("
        push rax
        push rbx
        push rcx
        push rdx
        push rsi
        push rdi
        push rbp
        push r8
        push r9
        push r10
        push r11
        push r12
        push r13
        push r14
        push r15

        " :::: "intel", "volatile");
    }};
}

macro_rules! popall {
    () => {{
        llvm_asm!("
        pop r15
        pop r14
        pop r13
        pop r12
        pop r11
        pop r10
        pop r9
        pop r8
        pop rbp
        pop rdi
        pop rsi
        pop rdx
        pop rcx
        pop rbx
        pop rax
        " :::: "intel", "volatile");
    }};
}
#[no_mangle]
#[inline(always)]
#[naked]
pub unsafe extern "C" fn isr0() -> () {
    pushall!();
    serial_println!("isr0 heree");
    PIC8259::sendEOI(0);
    popall!();
    llvm_asm!("iretq");
}
#[no_mangle]
#[inline(always)]
#[naked]
pub unsafe extern "C" fn isr1() -> () {
    pushall!();
    println!("{}",io_ports::inb(0x60));
    PIC8259::sendEOI(1);
    popall!();

    llvm_asm!("iretq");
}
#[no_mangle]
#[inline(always)]
#[naked]
pub unsafe extern "C" fn isr2() -> () {
    pushall!();
    println!("isr2 :(");
    PIC8259::sendEOI(2);
    PIC8259::sendEOI(10);
    popall!();
    llvm_asm!("iretq");
}
#[no_mangle]
#[inline(always)]
#[naked]
pub unsafe extern "C" fn double_fault() -> () {
    serial_println!("DOUBLE FAULT");
}
#[no_mangle]
#[inline(always)]
#[naked]
pub unsafe extern "C" fn div_zero() -> () {
    serial_println!("DIVIDE BY ZERO");
}
#[no_mangle]
#[inline(always)]
#[naked]
pub unsafe extern "C" fn general_protect_fault() -> () {
    serial_println!("GENERAL PROTECTION FAULT");
}
#[no_mangle]
#[inline(always)]
#[naked]
pub unsafe extern "C" fn segment_not_present() -> () {
    serial_println!("SEGMENT NOT PRESENT");
}
#[no_mangle]
#[inline(always)]
#[naked]
pub unsafe extern "C" fn invalid_tss() -> () {
    serial_println!("INVALID TSS");
}
#[no_mangle]
#[inline(always)]
#[naked]
pub unsafe extern "C" fn stack_segment_fault() -> () {
    serial_println!("STACK SEGMENT FAULT");
}
#[no_mangle]
#[inline(always)]
#[naked]
pub unsafe extern "C" fn page_fault() -> () {
    serial_println!("PAGE FAULT");
}