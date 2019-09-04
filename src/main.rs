extern crate xtrx_sys;

use std::ffi::CString;
use std::mem::size_of;
use std::os::raw::c_uint;
use xtrx_sys::{xtrx_dev, xtrx_open, xtrx_device_info_t, xtrx_discovery};

pub fn discovery() -> Vec<xtrx_device_info_t> {
    const MAX_DEVS: usize = 8;
    unsafe {
        let mut di = Vec::with_capacity(size_of::<xtrx_device_info_t>() * MAX_DEVS);
        let cnt = xtrx_discovery(di.as_mut_ptr(), MAX_DEVS);
        di.set_len(cnt as usize);
        di
    }
}

pub fn open_device(device: &str, flags: u32) -> *mut xtrx_dev {
    let c_device = CString::new(device).expect("Can't convert NULL pointer");
    unsafe {
        let mut output: *mut *mut xtrx_dev;
        xtrx_open(c_device.as_ptr(), flags as c_uint, output);
        *output
    }
}

fn main() {
    let dev_infos = discovery();
    println!("{:?}", dev_infos.len());
}
