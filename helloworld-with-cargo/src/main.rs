#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(feature="clippy", allow(not_unsafe_ptr_arg_deref))]

use std::ffi::{CString, CStr};
use std::env;
use std::os::raw::c_char;

// Add a + b. section

#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
    assert_eq!(add(10, -3), 7);
}

#[no_mangle]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Concat c + d. section

#[test]
fn test_concat() {
    assert_eq!(concat("a".to_string(), "b"), "ab");
}

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

// Callback twice. section

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

// Call Rust function via pointer. section

#[no_mangle]
pub fn get_callback_func() -> extern "C" fn(*const c_char) -> *mut c_char {
    callback_string
}

extern "C" fn callback_string(v: *const c_char) -> *mut c_char {
    let v = unsafe { CStr::from_ptr(v) };
    let result = format!("{} and ♡ from Rust", v.to_str().unwrap());
    CString::new(result).unwrap().into_raw() // memory leak maybe occured! ;P
}

// Call Rust struct method. section

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

// Call predefined js function from Rust. section

// in .cargo/config... #[link_args = "--js-library static/emcc-bind.js"]
extern "C" {
    fn exec_js_eval(str: *const c_char);
}

#[no_mangle]
pub fn exec_js_eval_c(str: *const c_char) {
    unsafe {
        exec_js_eval(str);
    }
}

// Use emscripten_sleep. section

extern "C" {
    fn emscripten_sleep(ms: i32);
}

#[no_mangle]
pub fn exec_sleep_c(ms: i32, callback: extern "C" fn()) {
    unsafe {
        emscripten_sleep(ms);
        callback();
    }
}

// Call async function synchronously in the Rust land. section

#[derive(Default)]
pub struct AsyncResultContainer {
    str: String,
}

impl AsyncResultContainer {
    pub fn new() -> Self {
        AsyncResultContainer { ..Default::default() }
    }
    pub fn set(&mut self, str: String) {
        self.str = str;
    }
}

#[no_mangle]
pub fn async_result_container_set(ptr: *mut AsyncResultContainer, v: *const c_char) {
    let mut container = unsafe { &mut *ptr };
    let v = unsafe { CStr::from_ptr(v) };
    let v = v.to_str().unwrap();
    container.str = v.to_string();
}

#[allow(improper_ctypes)]
extern "C" {
    fn exec_async(container: *mut AsyncResultContainer);
}

#[no_mangle]
pub fn exec_async_c(callback: extern "C" fn(*const c_char)) {
    let container = Box::into_raw(Box::new(AsyncResultContainer::new())) as
                    *mut AsyncResultContainer;
    unsafe {
        // JS process runs asynchronously, but it looks synchronous.
        exec_async(container);
        let container = Box::from_raw(container);
        let s = CString::new(container.str).unwrap().into_raw();
        callback(s);
        CString::from_raw(s);
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("2 arguments required.")
    }
    let a: i32 = args[1].parse().unwrap();
    let b: i32 = args[2].parse().unwrap();
    println!("result: {}", add(a, b));
}
