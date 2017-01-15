use std::ffi::CStr;
use std::str;

use libc;

#[no_mangle]
pub extern "C" fn rust_none_none() {
    println!("💩");
}

#[no_mangle]
pub extern "C" fn rust_int_none(arg: libc::c_int) {
    println!("{}", arg);
}

#[no_mangle]
pub extern "C" fn rust_string_none(arg: *const libc::c_char) {
    let c_str: &CStr = unsafe { CStr::from_ptr(arg) };
    let buf: &[u8] = c_str.to_bytes();
    let str_slice = str::from_utf8(buf).unwrap();
    println!("{}", str_slice);
}

#[no_mangle]
pub extern "C" fn rust_none_int() -> libc::c_int {
    42
}


// #[no_mangle]
// pub extern "C" fn return_from_rust(arg: *const libc::c_char) -> *mut libc::c_char {
//     let c_str = unsafe { CStr::from_ptr(arg) };
//     let buf: &[u8] = c_str.to_bytes();
//     let str_slice: &str = str::from_utf8(buf).unwrap();
//     let mut output_string: String = "Hello ".to_owned();
//     output_string.push_str(str_slice);
//     let cstr = CString::new(output_string).unwrap();
//     cstr.into_raw()
// }






// pub struct Test {
//     a: String,
// }
//
//
// #[no_mangle]
// pub extern "C" fn f1(c_ptr: *const libc::c_char) -> *mut Test {
//     let c_str = unsafe {
//         assert!(!c_ptr.is_null());
//         CStr::from_ptr(c_ptr)
//     };
//     let buf: &[u8] = c_str.to_bytes();
//     let str_slice: &str = str::from_utf8(buf).unwrap();
//     unsafe { mem::transmute(Box::new(Test { a: String::from(str_slice) })) }
// }
//
// #[no_mangle]
// pub extern "C" fn f2(cb: extern "C" fn(i32) -> *mut Test) {
//     let c_ptr = cb(42);
//     let test_object: Box<Test> = unsafe { mem::transmute(c_ptr) };
//     println!("{:?}", test_object.a);
// }
//
//
//
//
// use std::slice;
// use libc::{size_t, c_char};
//
// #[no_mangle]
// pub extern "C" fn get_strings(array: *const *const c_char, length: size_t) {
//     let values = unsafe { slice::from_raw_parts(array, length as usize) };
//     let strs: Vec<&str> = values.iter()
//         .map(|&p| unsafe { CStr::from_ptr(p) })  // iterator of &CStr
//         .map(|cs| cs.to_bytes())                 // iterator of &[u8]
//         .map(|bs| str::from_utf8(bs).unwrap())   // iterator of &str
//         .collect();
//     println!("{:?}", strs);
// }
