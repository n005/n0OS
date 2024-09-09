#![no_std]
#![no_main]

mod rendering;
mod io;

use core::{arch::asm, fmt::Write, panic};

use bootloader_api::{entry_point, info::FrameBufferInfo, BootInfo};
use rendering::framebuffer::{self, FrameBufferWriter};

bootloader_api::entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {

    let framebufferinfo = &boot_info.framebuffer.as_ref().unwrap().info();
    let width = &framebufferinfo.width;
    let height = &framebufferinfo.height;
    let pixelheight = &framebufferinfo.bytes_per_pixel;
    let pitch = &framebufferinfo.stride;
    let colorinfo = &framebufferinfo.pixel_format;

    let vga_buffer = boot_info.framebuffer.as_mut().unwrap().buffer_mut();

    let mut color = colorToU32(192, 21, 98, colorinfo);

    for i in 0..*width {
        for j in 0..*height {
            putpixel(vga_buffer, i, j, color, *pixelheight, *pitch);
        }
    }

    putpixel(
        vga_buffer,
        0,
        0,
        colorToU32(255, 255, 255, colorinfo),
        *pixelheight,
        *pitch,
    );
    putpixel(
        vga_buffer,
        *width - 1,
        *height - 1,
        colorToU32(255, 255, 255, colorinfo),
        *pixelheight,
        *pitch,
    );

    let mut framebufferwrite = FrameBufferWriter::new(vga_buffer, *framebufferinfo);

    framebufferwrite
        .write_str("Welcome to the N0OS ! \n My RUST Operation System!")
        .unwrap();

    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

fn putpixel(
    framebuffer: &mut [u8],
    x: usize,
    y: usize,
    color: u32,
    pixelheight: usize,
    pitch: usize,
) {
    let offset = (x * pixelheight) + (y * pitch * pixelheight);
    framebuffer[offset] = (color & 255) as u8;
    framebuffer[offset + 1] = ((color >> 8) & 255) as u8;
    framebuffer[offset + 2] = ((color >> 16) & 255) as u8;
}

fn colorToU32(r: u32, g: u32, b: u32, colorinfo: &bootloader_api::info::PixelFormat) -> u32 {
    let mut color32 = 0;
    match colorinfo {
        bootloader_api::info::PixelFormat::Rgb => {
            color32 = (b << 16) | (g << 8) | r;
        }
        bootloader_api::info::PixelFormat::Bgr => {
            color32 = (r << 16) | (g << 8) | b;
        }
        bootloader_api::info::PixelFormat::U8 => {
            color32 = r;
        }
        bootloader_api::info::PixelFormat::Unknown {
            red_position,
            green_position,
            blue_position,
        } => {
            color32 = (r << red_position) | (g << green_position) | (b << blue_position);
        }
        _ => {
            color32 = 0;
        }
    }
    color32
}
