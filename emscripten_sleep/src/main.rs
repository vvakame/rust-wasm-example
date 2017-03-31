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

fn main() {}
