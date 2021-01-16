use crate::vga_buffer;
use crate::keyboard::{Key, KeyType};
use alloc::string::String;
use crate:: filesystem;
use crate::heap;

pub static mut root_file_address: u64 = 0;
pub unsafe fn init(root_address: u64){
    print!(">");
    root_file_address = root_address;
    vga_buffer::update_cursor(1);
}
unsafe fn parse_cmd(cmd: String){
    let rootfile: &mut filesystem::VirtualDirectory = &mut*(root_file_address as *mut filesystem::VirtualDirectory); 
    vga_buffer::clear_cmd_row();
    println!("{}:\n", cmd);
    let mut split_cmd = cmd.split_ascii_whitespace();
    match split_cmd.next(){
        Some("pwd") => {pwd(rootfile)},
        Some("ls") => {ls(rootfile)},
        Some("help") => {help()},
        Some("cd") => {
            if let Some(arg) = split_cmd.next(){
                cd(rootfile, arg);
            }
            else{
                println!("Usage: cd <arg>");
            }
        },
        Some("cat") => {
            if let Some(arg) = split_cmd.next(){
                cat(rootfile, arg);
            }
            else{
                println!("Usage: cat <arg>");
            }
        },
        _ => {
            println!("Command not found");
            println!("Try writing 'help' for list of avaiable commands");
        }
    }
    print!(">");
    vga_buffer::update_cursor(1);
}
pub unsafe fn pwd(rootfile: &mut filesystem::VirtualDirectory){
    println!("{}", rootfile.path);
}

pub unsafe fn ls(rootfile: &mut filesystem::VirtualDirectory){   
    println!("{:?}", rootfile.get_filenames());

    
}
pub unsafe fn cd(rootfile: &mut filesystem::VirtualDirectory, dir: &str){
    if let Some(x) = rootfile.get_child_by_name(dir) {
        root_file_address = x as *const _ as u64;
        // serial_println!("a: {:p}", &rootfile);
        // serial_println!("b: {:p}", &rootfile.directories[0]);
        // serial_println!("c: {:x}", root_file_address);
    }
    else{
        println!("No directories named '{}' exists", dir);
    }
    
}
pub unsafe fn cat(rootfile: &mut filesystem::VirtualDirectory, filename: &str){    
    if let Some(x) = rootfile.get_file_by_name(filename) {
        let file = &*x;
        println!("{}:", filename);
        println!("{}", file.get_content());
    }
    else{
        println!("No file named '{}' exists", filename);
    }
    
}
pub fn help(){    
    println!("Avaiable commands:");
    println!("help, pwd, ls");
    
}
pub unsafe fn print_to_cli(key: Key) {
    match key.key_type {
        KeyType::Enter => {
            //serial_println!("cli: {}##", vga_buffer::get_cmd());
            parse_cmd(vga_buffer::get_cmd());
        },
        _ => {
            if let Some(k) = key.toChar() {
                print!("{}", k);
            }
        }
    }
}