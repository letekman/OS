#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(naked_functions)]
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
    println!("Hello World{} 123: {}", "!",123);
    serial_println!("YO");
    unsafe{
        PIC8259::set_irq_mask(0);
        PIC8259::set_irq_mask(1);
        PIC8259::set_irq_mask(2);
        idt::load_idt();
        //stack_overflow();
    }
    //let mut i = 0;
    loop {
        // unsafe{

        //     llvm_asm!("hlt" :::: "volatile");
        // }
        //println!("hey {} ", i);
        //i = i + 1;
    }
}
#[allow(unconditional_recursion)]
fn stack_overflow(){
    stack_overflow();
    volatile::Volatile::new(0).read();
}
/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    //println!("{}", info);
    loop {}
}
