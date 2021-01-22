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


extern crate alloc;


#[macro_use]
mod serial;
#[macro_use]
mod vga_buffer;
mod io_ports;
mod isr_handlers;
mod keyboard;
mod pic8259;
pub mod heap;
mod tar_driver;
mod filesystem;
mod cli;

mod idt;

extern {
    static mut _RootFile_Size: u64;
    static mut _RootFile: u64;
} 

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Booted");
    unsafe {
        
        heap::init_heap();
        pic8259::stop_raising_irq();
        idt::load_idt();
        println!("{:#x}", _RootFile_Size);
        println!("{:#p}", &_RootFile);
        let file = tar_driver::File::new(&mut _RootFile).unwrap();
        let rootfile = filesystem::init_vfs(file);
        for _i in 0..25{
            println!();
        }
        print_ascii_art(&rootfile);

        cli::init(&rootfile as *const _ as u64);

        loop {}
    }
}
pub unsafe fn print_ascii_art(rootfile: &filesystem::VirtualDirectory){
    println!("{}", rootfile.directories[0].files[0].get_content());
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
