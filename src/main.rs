#![no_main]
#![no_std]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// fn main() {
//     // println!("Hello, world!");
// }

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // let vga_buffer = 0xb8000 as *mut u8;
    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }

    vga_buffer::print_something();
    loop {}
}