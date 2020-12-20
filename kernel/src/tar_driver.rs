struct tar_header {
    filename: [char; 100],
    filemode: u64,
    owner_id: u64,
    group_id: u64,
    file_size: [char; 12],
    last_modification: [char; 12],
    checksum: u64,
    type_flag: char,
    linked_file_name: [char; 100],
    ustar_indicator: [char; 6],
    ustar_version: [char; 2],
    owner_name: [char; 32],
    owner_group_name: [char; 32],
    device_major_number: u64,
    device_minor_number: u64,
    filename_prefix: [char; 155]
}

pub fn oct_to_bin(file_size: [char;12]) -> u32{
    let mut n: u32 = 0;
    for i in (0..11).rev() {
        n *= 8;
        println!("{}, {}", file_size[i], file_size[i] as u32 - 0x30 as u32);
        n += file_size[i] as u32 - 0x30 as u32;
    }
    return n;
}