use std::ffi::{CString, CStr};
use std::os::raw::c_char;

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

fn main() {}

// emscripten組込の非同期な関数を呼ぶコードがないとasync系ヘルパが生成されずに動かない
// 何かしら回避する方法があると思うんだけど今のところ不明
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
