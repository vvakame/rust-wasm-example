use std::ffi::{CString, CStr};
use std::os::raw::c_char;

pub fn concat(a: String, b: &str) -> String {
    a + b
}

#[no_mangle]
pub extern "C" fn concat_c(a: *const c_char, b: *const c_char) -> *mut c_char {
    let a = unsafe { CStr::from_ptr(a) };
    let b = unsafe { CStr::from_ptr(b) };
    let a = a.to_str().unwrap();
    let b = b.to_str().unwrap();

    let result = concat(a.to_string(), b);
    CString::new(result).unwrap().into_raw() // memory leak maybe occured! ;P
}

fn main() {}
