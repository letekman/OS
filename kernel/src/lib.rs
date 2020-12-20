#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(naked_functions)]
#![feature(fmt_as_str)]
#![feature(core_intrinsics)]
#![feature(asm)]
#![feature(alloc_error_handler)] 
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(new_uninit)]
use core::panic::PanicInfo;
use alloc::vec::Vec;

extern crate alloc;


#[macro_use]
mod serial;
#[macro_use]
mod vga_buffer;
mod io_ports;
mod isr_handlers;
mod keyboard;
mod PIC8259;
pub mod heap;
mod tar_driver;


use alloc::boxed::Box;
mod idt;
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Booted");
    unsafe {
        
        heap::init_heap();
        PIC8259::set_irq_mask(0);
        PIC8259::set_irq_mask(1);
        PIC8259::set_irq_mask(2);
        idt::load_idt();
        
        //divide_by_zero();
        //unsafe { *(0xdeadbeaf as *mut u64) = 42 };
        println!("10: {}", tar_driver::oct_to_bin(['0','0','0','0','0','0','0','0','0','1','2','\0']))
    }

    loop {
        
    }
}
fn divide_by_zero() {
    unsafe {
        llvm_asm!("mov dx, 0; div dx" ::: "ax", "dx" : "volatile", "intel")
    }
}
/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout);
}
