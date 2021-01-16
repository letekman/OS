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
use alloc::string::String;


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
mod filesystem;
mod cli;


use alloc::boxed::Box;
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
        PIC8259::set_irq_mask(0);
        PIC8259::set_irq_mask(1);
        PIC8259::set_irq_mask(2);
        idt::load_idt();
        println!("{:#x}", _RootFile_Size);
        println!("{:#p}", &_RootFile);
        //divide_by_zero();

        let file = tar_driver::File::new(&mut _RootFile).unwrap();
        let rootfile = filesystem::initVFS(file);
        // println!("rootfile {}", rootfile.name);
        // println!("rootfile {}", rootfile.name);

        // // for file in rootfile.files {
        // //     println!("file: {:#?}", file);
        // // }
        // // for dir in rootfile.directories {
        // //     serial_println!("NEW DIRECTORY");
        // //     serial_println!("dir: {:#?}", dir);
        // //     for file in dir.files {
        // //         serial_println!("file: {:#?}", file);
        // //     }
        // // }
        for i in 0..25{
            println!();
        }
        println!("{}", rootfile.directories[0].files[0].get_content());

        cli::init(&rootfile as *const _ as u64);
        //println!("NICE");
        // //unsafe { *(0xdeadbeaf as *mut u64) = 42 };
        // let file2 = tar_driver::File::new(file.next, "anotherfile.txt").unwrap();
        // let file3 = tar_driver::File::new(file2.next, "third.txt").unwrap();
        // let file4 = tar_driver::File::new(file3.next, "4th.txt");

        
        // serial_println!("file1: {:#?}", file);
        // serial_println!("file2: {:#?}", file2);
        // serial_println!("file2: {:#?}", file3);
        // match file4 {
        //     Some(x) => serial_println!("file4: {:#?}", x),
        //     None => println!("EVEN MORE SUCCESS")
        // }
        // //println!("head: {}", _RootFile);
      
        //println!("10: {}", tar_driver::oct_to_bin(['2','1','0','0','0','0','0','0','0','0','0','\0']))
        

    loop {
        
    }
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
