use std::os::raw::c_char;

#[no_mangle]
pub unsafe extern "C" fn some_function(time: i64, _data: *const c_char) -> i64 {
    time
}
