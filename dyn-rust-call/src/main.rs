use std::ffi::{CString, CStr};
use std::os::raw::c_char;

#[no_mangle]
pub fn get_callback_func() -> extern "C" fn(*const c_char) -> *mut c_char {
    callback_string
}

extern "C" fn callback_string(v: *const c_char) -> *mut c_char {
    let v = unsafe { CStr::from_ptr(v) };
    let result = format!("{} and ♡ from Rust", v.to_str().unwrap());
    CString::new(result).unwrap().into_raw() // memory leak maybe occured! ;P
}

fn main() {}
