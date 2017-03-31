use std::ffi::{CString, CStr};
use std::os::raw::c_char;

fn callback_twice<F>(v: &str, mut f: F)
    where F: FnMut(&str)
{
    f(v);
    f(v);
}

#[no_mangle]
pub extern "C" fn callback_twice_c(v: *const c_char, f: extern "C" fn(*const c_char)) {
    let v = unsafe { CStr::from_ptr(v) };
    let v = v.to_str().unwrap();
    callback_twice(v, |v| {
        let s = CString::new(v).unwrap().into_raw();
        f(s);
        unsafe { CString::from_raw(s) };
    });
}

fn main() {}
