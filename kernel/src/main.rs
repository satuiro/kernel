#![no_std]
#![no_main]

mod framebuffer;

use core::panic::PanicInfo;
use bootloader_api::info::FrameBufferInfo;
use bootloader_x86_64_common::logger::LockedLogger;
use conquer_once::spin::OnceCell;


pub(crate) static LOGGER: OnceCell<LockedLogger> = OnceCell::uninit();

pub(crate) fn init_logger(buffer: &'static mut [u8], info: FrameBufferInfo) {
    let logger = LOGGER.get_or_init(move || LockedLogger::new(buffer, info, true, false));
    log::set_logger(logger).expect("Logger alreasy set");
    log::set_max_level(log::LevelFilter::Trace);
    log::info!("The mathematics says that 2 + 2 = {}", 2 + 2);
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

bootloader_api::entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    // Drawing a simple RGB pixel over the kernel
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let color = framebuffer::Color {
            red: 23,
            green: 162,
            blue: 255,
        };
        for x in 0..100 {
            for y in 0..100 {
                let position = framebuffer::Position {
                    x: 20 + x,
                    y: 100 + y,
                };
                framebuffer::set_pixel_in(framebuffer, position, color);
            }
        }
    }

    let frame_buffer_optional = &mut boot_info.framebuffer;
    let frame_buffer_option = frame_buffer_optional.as_mut();
    let frame_buffer_struct = frame_buffer_option.unwrap();
    let frame_buffer_info = frame_buffer_struct.info().clone();

    // get the framebuffer's mutable raw byte slice
    let raw_frame_buffer = frame_buffer_struct.buffer_mut();

    // finally, initialize the logger using the last two variables
    init_logger(raw_frame_buffer, frame_buffer_info);

    loop {}
}