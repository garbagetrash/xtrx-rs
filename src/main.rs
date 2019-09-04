extern crate xtrx_sys;

use std::mem::size_of;
use xtrx_sys::{xtrx_device_info_t, xtrx_discovery};

fn main() {
    const MAX_DEVS: usize = 8;
    let cnt = unsafe {
        let mut di = Vec::with_capacity(size_of::<xtrx_device_info_t>() * MAX_DEVS);
        xtrx_discovery(di.as_mut_ptr(), MAX_DEVS)
    };
    println!("{:?}", cnt);
}
