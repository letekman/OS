use crate::vga_buffer;
use crate::keyboard::{Key, KeyType};
use alloc::string::String;
use crate:: filesystem;
pub static mut ROOT_FILE_ADDRESS: u64 = 0;
pub unsafe fn init(root_address: u64){
    print!(">");
    ROOT_FILE_ADDRESS = root_address;
    vga_buffer::update_cursor(1);
}
unsafe fn parse_cmd(cmd: String){
    let rootfile: &mut filesystem::VirtualDirectory = &mut*(ROOT_FILE_ADDRESS as *mut filesystem::VirtualDirectory); 
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
                println!("Usage: cd <foldername> or cd .. to go up one directory");
            }
        },
        Some("cat") => {
            if let Some(arg) = split_cmd.next(){
                cat(rootfile, arg);
            }
            else{
                println!("Usage: cat <filename>");
            }
        },
        Some("echo") => {
            if let Some(arg) = split_cmd.next(){
                print!("{}", arg);
                while let Some(next_word) = split_cmd.next(){
                    print!(" {}", next_word);
                }
            }
            println!();
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
    if dir == ".."{
        if rootfile.name != "root" {
            ROOT_FILE_ADDRESS = rootfile.parent as u64;
        }
        return
    }
    if let Some(x) = rootfile.get_child_by_name(dir) {
        ROOT_FILE_ADDRESS = x as *const _ as u64;
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
    println!("help, pwd, ls, cat, cd, echo");
    
}
pub unsafe fn print_to_cli(key: Key) {
    match key.key_type {
        KeyType::Enter => {
            parse_cmd(vga_buffer::get_cmd());
        },
        _ => {
            if let Some(k) = key.to_char() {
                print!("{}", k);
            }
        }
    }

}