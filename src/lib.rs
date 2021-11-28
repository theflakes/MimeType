#![cfg(windows)]

extern crate tree_magic;
extern crate libc;

use libc::{c_char};
use std::ffi::{CStr, CString};
use std::path::Path;
use std::fs::metadata;


#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn get_mimetype(target_file: *const c_char) -> *mut c_char {
    let c_str : &CStr = unsafe {
        assert!(!target_file.is_null());

        CStr::from_ptr(target_file)
    };
    
    let temp = c_str.to_str().unwrap();
    let file_path = Path::new(&temp);
    let mut mtype = CString::new("").unwrap();
    if Path::new(file_path).exists() { 
        let md = metadata(file_path).unwrap();
        if md.is_file() {
            mtype = CString::new(tree_magic::from_filepath(file_path)).expect("ERROR");
        }
    }

    return mtype.into_raw()
}