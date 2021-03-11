//! Directly plug a `main` symbol instead of using `#[entry]`

#![deny(warnings)]
#![no_main]
#![no_std]

use neutron_star_rt::*;
use neutron_star::*;
extern crate panic_halt;


#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    println!("Hello world! {}, {}", 0, 2);
    __exit(5);
}


    /*
    let msg = "Hello World!";
    __push_costack(msg.as_ptr(), msg.len());
    let count = 1 as u8;
    let count_ptr: *const u8 = &count;
    __push_costack(count_ptr, 1);
    __system_call(4, 2);
    */
/*
#![no_std]
#![no_main]

#![crate_type = "staticlib"]

//use neutron_star_rt::*;

use neutron_star_rt::*;


#[no_mangle]
#[export_name = "main"]
#[link_section = ".main"]
pub unsafe extern "C" fn main() -> u32{
    let foo = neutron_star_rt::__entropy(100);
    foo
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    unsafe {
        neutron_star_rt::__exit(0);
    }
}
*/

