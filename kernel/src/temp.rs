
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
        push rsp
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
        pop rsp
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
#[naked]
pub extern "C" fn isr0() -> () {

        pushall!();
        print!(".");
        PIC8259::sendEOI(0);
        popall!();
        llvm_asm!("iretq");
    }
}
#[naked]
pub extern "C" fn isr1() -> () {
    unsafe{

        pushall!();
        let scancode: u8 = io_ports::inb(0x60);
        print!("{}", scancode);
        PIC8259::sendEOI(1);
        popall!();
    
        llvm_asm!("iretq");
    }
}
#[naked]
pub extern "C" fn isr2() -> () {
    unsafe{

        pushall!();
        println!("isr2 :(");
        PIC8259::sendEOI(2);
        PIC8259::sendEOI(10);
        popall!();
        llvm_asm!("iretq");
    }
}

#[naked]
pub extern "C" fn div_zero() -> () {
    serial_println!("DIVIDE BY ZERO");
}
#[naked]
pub extern "C" fn debug() -> () {
    serial_println!("DEBUG");
}
#[naked]
pub extern "C" fn non_maskable_interrupt() -> () {
    serial_println!("NON_MASKABLE_INTERRUPT");
}
#[naked]
pub extern "C" fn breakpoint() -> () {
    serial_println!("breakpoint");
}
#[naked]
pub extern "C" fn overflow() -> () {
    serial_println!("overflow");
}
#[naked]
pub extern "C" fn bound_range_exceeded() -> () {
    serial_println!("bound_range_exceeded");
}
#[naked]
pub extern "C" fn invalid_opcode() -> () {
    serial_println!("invalid_opcode");
}
#[naked]
pub extern "C" fn device_not_avaiable() -> () {
    serial_println!("device_not_avaiable");
}
#[naked]
pub extern "C" fn double_fault() -> () {
    serial_println!("DOUBLE FAULT");
}
#[naked]
pub extern "C" fn invalid_tss() -> () {
    serial_println!("INVALID TSS");
}
#[naked]
pub extern "C" fn segment_not_present() -> () {
    serial_println!("SEGMENT NOT PRESENT");
}
#[naked]
pub extern "C" fn stack_segment_fault() -> () {
    serial_println!("STACK SEGMENT FAULT");
}
#[naked]
pub extern "C" fn general_protect_fault() -> () {
    serial_println!("GENERAL PROTECTION FAULT");
}
#[naked]
pub extern "C" fn page_fault() -> () {
    serial_println!("PAGE FAULT");
}

#[naked]
pub extern "C" fn x87_floating_point() -> () {
    serial_println!("x87_floating_point");
}

#[naked]
pub extern "C" fn alignment_check() -> () {
    serial_println!("alignment_check");
}

#[naked]
pub extern "C" fn machine_check() -> () {
    serial_println!("machine check");
}
#[naked]
pub extern "C" fn simd() -> () {
    serial_println!("simd");
}
#[naked]
pub extern "C" fn virtualization() -> () {
    serial_println!("virtualization");
}
#[naked]
pub extern "C" fn security_exception() -> () {
    serial_println!("security_exception");
}