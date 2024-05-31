#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

bootloader_api::entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    loop {}
}