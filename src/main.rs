#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// fn main() {
//     // println!("Hello, world!");
// }

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}