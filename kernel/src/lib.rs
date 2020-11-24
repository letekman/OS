#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(naked_functions)]
#![feature(fmt_as_str)]
#![feature(core_intrinsics)]
use core::panic::PanicInfo;

#[macro_use]
mod serial;
#[macro_use]
mod vga_buffer;
mod io_ports;
mod isr_handlers;
mod PIC8259;



mod idt;
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Booted");
    unsafe{
        PIC8259::set_irq_mask(0);
        PIC8259::set_irq_mask(1);
        PIC8259::set_irq_mask(2);
        idt::load_idt();
        
        println!("Hello from Rust");
    }
    loop {
    }
}
/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
