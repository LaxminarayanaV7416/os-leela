#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

/// Entry point for the OS
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to OS-Leela!");
    println!("An experimental OS built completely with Rust");
    println!();
    println!("System initialized successfully.");

    loop {}
}
