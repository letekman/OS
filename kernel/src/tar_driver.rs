use alloc::string::String;
use core::ptr;
use core::fmt;
#[derive(Copy, Clone)]
#[repr(packed)]
pub struct Tar_header {
    pub filename: [u8; 100],
    pub filemode: u64,
    pub owner_id: u64,
    pub group_id: u64,
    pub file_size: [u8; 12],
    pub last_modification: [u8; 12],
    pub checksum: u64,
    pub type_flag: u8,
    pub linked_file_name: [u8; 100],
    pub ustar_indicator: [u8; 6],
    pub ustar_version: [u8; 2],
    pub owner_name: [u8; 32],
    pub owner_group_name: [u8; 32],
    pub device_major_number: u64,
    pub device_minor_number: u64,
    pub filename_prefix: [u8; 155]
}

#[derive(Copy, Clone)]
pub struct File{ 
    pub header: Tar_header,
    pub memory_address: *mut u64,
    pub size: u32,
    pub next: *mut u64
}
impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("File")
         .field("address", &self.memory_address)
         //.field("name", &self.name)
         .field("size", &self.size)
         //.field("content", &self.content)
         .field("next", &self.next)
         .finish()
    }
}
impl File{
    pub fn new(memory_location: *mut u64) -> Option<File> {
        unsafe {
            let ptr: *mut Tar_header = memory_location as *mut Tar_header;
            let reference = *ptr;
            let decimal_size = oct_to_bin(reference.file_size);
            let segments = (511 + decimal_size)/512;
            if *memory_location == 0 {
                return None
            }
            use core::str;
            use core::slice;
            Some(
                File {
                    header: reference,
                    memory_address: memory_location,
                    //name: String::from(String::from_utf8(reference.filename.to_vec()).unwrap().trim()),
                    size: decimal_size,
                    //content: String::from(String::from_utf8(slice::from_raw_parts(memory_location.offset(64) as *mut u8, decimal_size as usize).to_vec()).unwrap().trim()),
                    next: memory_location.offset((64 * (1 + segments) as isize))
                }
            )
        }
    }
}
pub struct FileIter{
    curAddress: *mut u64,
}
impl FileIter {
    pub fn new(address: *mut u64) -> FileIter {
        FileIter {
            curAddress: address
        }
    }
}
impl Iterator for FileIter {

    type Item = File;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(x) = File::new(self.curAddress){
            self.curAddress = x.next;
            return Some(x)
        }
        return None
    }
}
pub fn oct_to_bin(file_size: [u8;12]) -> u32{
    let mut n: u32 = 0;
    for i in (0..11){
        n *= 8;
        n += file_size[i] as u32 - 0x30 as u32 ;
    }

    return n;
}