use alloc::vec::Vec;
use crate::tar_driver::File;
use crate::tar_driver::FileIter;
use alloc::string::String;
use core::iter::Enumerate;
use core::slice;
use core::fmt;
pub enum FileType {
    File,
    Directory
}
pub struct VirtualFile {
    pub file: File,
    pub name: String
}
impl VirtualFile {
    pub fn new(file: File, name: &str) -> VirtualFile {
        VirtualFile {
            file: file,
            name: String::from(name)
        }
    }
    pub unsafe fn get_content(&self) -> String{
        String::from(String::from_utf8(slice::from_raw_parts(self.file.memory_address.offset(64) as *mut u8, self.file.size as usize).to_vec()).unwrap().trim_end_matches('\u{0}'))
    }
    pub unsafe fn get_name(&self) -> String{
        String::from(String::from_utf8(self.file.header.filename.to_vec()).unwrap().trim_end_matches('\u{0}'))
    }
    pub unsafe fn get_short_name(&self) -> String{
        String::from(self.name.trim_end_matches('\u{0}'))
    }
}
impl fmt::Debug for VirtualFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe{

            f.debug_struct("File")
             .field("name", &self.get_name())
             //.field("name", &self.name)
             .field("content", &self.get_content())
             .finish()
        }
    }
}
pub struct VirtualDirectory {
    pub name: String,
    pub path: String,
    pub parent: *const VirtualDirectory,
    pub directories: Vec<VirtualDirectory>,
    pub files: Vec<VirtualFile>
}
impl fmt::Debug for VirtualDirectory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Directory")
         .field("name", &self.name)
         .field("path", &self.path)
         .finish()
    }
}
impl VirtualDirectory {
    pub fn new(name: &str, path:  &str, parent: *mut VirtualDirectory) -> VirtualDirectory {
        VirtualDirectory {
            name: String::from(name),
            path: String::from(path),
            parent: parent,
            directories: Vec::new(),
            files: Vec::new()
        }
    }
    pub fn contains_folder(&mut self, name: &'static str) -> bool {
        for dir in &self.directories {
            if dir.name == name {
                return true;
            }
        }
        return false;
    }
    pub fn get_child_by_name(&mut self,  name: &str) -> Option<*mut VirtualDirectory> {
        
        for dir in self.directories.iter_mut() {

            if dir.name == *name {
                return Some(dir)
            }
        }
        return None
    }
    pub unsafe fn get_file_by_name(&mut self,  name: &str) -> Option<*mut VirtualFile> {
        for file in self.files.iter_mut() {
            if file.get_short_name() == name {
                return Some(file)
            }
        }
        return None
    }
    pub unsafe fn get_filenames(&mut self) -> Vec<String> {
        let mut vector: Vec<String> = Vec::new();
        for dir in self.directories.iter_mut() {
            let mut name = dir.name.clone();
            name.push('*');
            vector.push(name);
        }
        for file in self.files.iter_mut() {
            vector.push(String::from(file.get_short_name().as_str()));
        }
        return vector
    }
}
pub unsafe fn initVFS(root: File) -> VirtualDirectory{
    let mut rootDir = VirtualDirectory::new("root", "root/", 0 as *mut VirtualDirectory);
    let mut i = 0;
    let iter: FileIter = FileIter::new(root.memory_address);
    for file in iter {

        if i == 0 {
            i += 1;
            continue;
        }


        let long_name = String::from(String::from_utf8(file.header.filename.to_vec()).unwrap().trim());
        let paths: Vec<String> = long_name.split('/').map(String::from).collect();
        
        let count = paths.len();
        let mut curDir: *mut VirtualDirectory= &mut rootDir;
        let mut j = 0;

        for name in paths {

            // if name.trim_end_matches('\u{0}') == "hello.txt"{
            //     serial_println!("HEY!");
            //     continue;
            // }
            if j == 0 || name.trim_end_matches('\u{0}') == ""{
                j += 1;
                continue;
            }
            if j == count - 1 {
                if file.header.type_flag == 48 || file.header.type_flag == 0 {
                    (*curDir).files.push(VirtualFile::new(file, name.as_str()));
                }
                else if file.header.type_flag == 53 {
                    if let Some(folder) = (*curDir).get_child_by_name(&name){
                    }
                    else {
                        let mut newPath = String::from((*curDir).path.clone());
                        newPath.push_str(name.as_str());
                        newPath.push_str("/");
        
                        (*curDir).directories.push(VirtualDirectory::new(&name, &newPath, curDir));
                    }
                }
                break;
            }
            if let Some(folder) = (*curDir).get_child_by_name(&name){
                curDir = folder;

            }
            else {
                let mut newPath = String::from((*curDir).path.clone());
                newPath.push_str(name.as_str());
                newPath.push_str("/");

                (*curDir).directories.push(VirtualDirectory::new(&name, &newPath, curDir));
            }
            j += 1;
        }

    i += 1;
    }

    rootDir
}