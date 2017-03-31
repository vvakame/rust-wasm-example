use std::ffi::{CString, CStr};
use std::os::raw::c_char;

#[derive(Default)]
pub struct StringContainer {
    list: Vec<String>,
}

impl StringContainer {
    pub fn new() -> Self {
        StringContainer { list: Vec::new() }
    }
    pub fn append(&mut self, str: String) {
        self.list.push(str);
    }
    pub fn concat_with(&self) -> String {
        if self.list.is_empty() {
            "from Rust\n".to_string()
        } else {
            self.list.join(" with Rust\n") + " with Rust\n"
        }
    }
}

#[no_mangle]
pub fn new_string_container() -> *mut StringContainer {
    Box::into_raw(Box::new(StringContainer::new())) as *mut StringContainer
}

#[no_mangle]
pub fn string_container_append(ptr: *mut StringContainer, v: *const c_char) {
    let mut container = unsafe { &mut *ptr };
    let v = unsafe { CStr::from_ptr(v) };
    let v = v.to_str().unwrap();
    container.append(v.to_string());
}

#[no_mangle]
pub fn string_container_concat_with(ptr: *mut StringContainer) -> *mut c_char {
    let container = unsafe { &mut *ptr };
    let result = container.concat_with();
    CString::new(result).unwrap().into_raw() // memory leak maybe occured! ;P
}

#[no_mangle]
pub fn drop_string_container(ptr: *mut StringContainer) {
    unsafe { Box::from_raw(ptr) };
}

fn main() {}
