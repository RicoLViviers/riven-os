#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
// Entry point
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    println!("Hello World! \n");
    println!("Some random numbers: {} {}", 42, 1.338);
    panic!("Some text");
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
