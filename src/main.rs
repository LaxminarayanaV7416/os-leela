//! OS-Leela: An experimental operating system built with Rust
//!
//! This is the main entry point for the OS. It sets up the bare-metal
//! environment and initializes basic I/O capabilities.

#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

/// This function is called on panic.
///
/// In a bare-metal environment, we need to define our own panic handler
/// since the standard library's panic handler is not available.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

/// Entry point for the OS
///
/// This function is called by the bootloader after the system boots.
/// It never returns (indicated by the `!` return type).
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to OS-Leela!");
    println!("An experimental OS built completely with Rust");
    println!();
    println!("System initialized successfully.");

    loop {}
}
