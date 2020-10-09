use std::os::raw::{c_char, c_int, c_uint};
use std::ffi::{CStr, CString};

#[no_mangle]
pub extern "C" fn p() {
    println!("call println in rust code");
}

#[no_mangle]
pub extern "C" fn hello(name: *const c_char) {
    assert!(!name.is_null());
    let c_str = unsafe {
        CStr::from_ptr(name)
    };
    println!("hello {:?}", c_str);
}

#[no_mangle]
pub extern "C" fn repeat_hi(times: c_int) -> *mut c_char {
    let mut res = String::new();
    res.extend(std::iter::repeat("hi").take(times as usize));
    CString::new(res).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn free_cstr(str: *mut c_char) {
    // 在Rust函数内创建的堆内存变量，C语言并不会自动释放，所以还需要在Rust侧提供一个API去释放堆内存
    if str.is_null() {
        return;
    }
    unsafe {
        CString::from_raw(str);
    };
}

#[no_mangle]
pub extern "C" fn sum_of_positive(arr: *const c_int, len: c_uint) -> c_int {
    if arr.is_null() {
        return c_int::from(-1);
    }
    let nums = unsafe {
        std::slice::from_raw_parts(arr, len as usize)
    };
    let sum: i32 = nums.iter().filter(|&x| x.is_positive()).sum();
    c_int::from(sum)
}

#[derive(Debug)]
#[repr(C)]
pub struct Point {
    x: i32,
    y: i32,
}

#[no_mangle]
pub extern "C" fn print_point(point: Point) {
    dbg!(point);
}
