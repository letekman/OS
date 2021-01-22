use alloc::vec::Vec;
use crate::tar_driver::File;
use crate::tar_driver::FileIter;
use alloc::string::String;
use core::slice;
use core::fmt;
pub struct VirtualFile {
    pub file: File,
    pub name: String
}
impl VirtualFile {
    pub fn new(file: File, name: &str) -> VirtualFile {
        VirtualFile {
            file: file,
            name: String::from(name).clone()
        }
    }
    pub unsafe fn get_content(&self) -> String{
        String::from(String::from_utf8(slice::from_raw_parts(self.file.memory_address.offset(64) as *mut u8, self.file.size as usize).to_vec()).unwrap().trim_end_matches('\u{0}')).clone()
    }
    pub unsafe fn get_name(&self) -> String{
        String::from(String::from_utf8(self.file.header.filename.to_vec()).unwrap().trim_end_matches('\u{0}')).clone()
    }
    pub unsafe fn get_short_name(&self) -> String{
        String::from(self.name.trim_end_matches('\u{0}')).clone()
    }
}
impl fmt::Debug for VirtualFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe{

            f.debug_struct("File")
             .field("name", &self.get_name())
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
            name: String::from(name).clone(),
            path: String::from(path).clone(),
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
pub unsafe fn init_vfs(root: File) -> VirtualDirectory{
    let mut root_dir = VirtualDirectory::new("root", "root/", root.memory_address as *mut VirtualDirectory);
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
        let mut cur_dir: *mut VirtualDirectory= &mut root_dir;
        let mut j = 0;

        for name in paths {
            if j == 0 || name.trim_end_matches('\u{0}') == ""{
                j += 1;
                continue;
            }
            if j == count - 1 {
                if file.header.type_flag == 48 || file.header.type_flag == 0 {
                    (*cur_dir).files.push(VirtualFile::new(file, name.as_str()));
                }
                else if file.header.type_flag == 53 {
                    if let Some(_folder) = (*cur_dir).get_child_by_name(&name){
                    }
                    else {
                        let mut new_path = String::from((*cur_dir).path.clone());
                        new_path.push_str(name.as_str());
                        new_path.push_str("/");
        
                        (*cur_dir).directories.push(VirtualDirectory::new(&name, &new_path, cur_dir));
                    }
                }
                break;
            }
            if let Some(folder) = (*cur_dir).get_child_by_name(&name){
                cur_dir = folder;

            }
            else {
                let mut new_path = String::from((*cur_dir).path.clone());
                new_path.push_str(name.as_str());
                new_path.push_str("/");

                (*cur_dir).directories.push(VirtualDirectory::new(&name, &new_path, cur_dir));
            }
            j += 1;
        }

    i += 1;
    }

    root_dir
}