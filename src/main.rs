#![feature(plugin)]
// Make linter fail for every warning
#![plugin(clippy)]
#![deny(clippy)]

#![allow(non_camel_case_types)]
#![allow(enum_variant_names)]


extern crate libc;
extern crate va_list;
extern crate xbee;

use xbee::*;
use std::ffi::{CString, CStr};
use std::ptr;
use libc::time_t;
use libc::timespec;
use std::os::raw::*;
use std::mem::*;
use std::option::Option;
use std::default::Default;
use va_list::VaList;
use libc::FILE;


fn main() {
    //const XBEE_TYPE: &'static str = "xbeeZB";
    let d: *mut c_void;
    let mut xbee: *mut Struct_xbee = ptr::null_mut();
    let con: *mut Struct_xbee_con;
    let address: Struct_xbee_conAddress;
    let ret: xbee_err;
    //let mut ret2: i32 = 100;
    
    let c_str_xbee_type = CString::new("xbeeZB").unwrap();
    let c_str_xbee_usb_dongle = CString::new("/dev/tty.usbserial-141").unwrap();

    assert!(xbee.is_null());
    
    unsafe {
        ret = xbee_setup(&mut xbee, c_str_xbee_type.as_ptr(), c_str_xbee_usb_dongle.as_ptr(), 57600);
    }

    assert!(xbee.is_null());
    
    //println!("ret2 = {}", ret2);
}
