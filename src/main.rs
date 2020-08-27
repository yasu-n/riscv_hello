#![feature(global_asm)]
#![feature(lang_items)]
#![no_std]
#![no_main]

global_asm!(include_str!("boot.s"));

use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub fn __start_hello() -> ! {
    let uart0 = 0x1000_0000 as *mut u8;
    for c in b"Hello, Rust\n".iter() {
        unsafe {
            *uart0 = *c as u8;
        }
    }

    loop {}
}

#[no_mangle]
pub fn abort() -> ! {
    loop {}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
