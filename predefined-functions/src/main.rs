use std::os::raw::c_char;

extern "C" {
    fn exec_js_eval(str: *const c_char);
}

#[no_mangle]
pub fn exec_js_eval_c(str: *const c_char) {
    unsafe {
        exec_js_eval(str);
    }
}

fn main() {}
