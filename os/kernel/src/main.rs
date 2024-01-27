#![no_std]
#![no_main]

use core::arch::asm;

use bootloader_api::{entry_point, BootInfo};

bootloader_api::entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    let width = &boot_info.framebuffer.as_ref().unwrap().info().width;
    let height = &boot_info.framebuffer.as_ref().unwrap().info().height;
    let pixelheight = &boot_info.framebuffer.as_ref().unwrap().info().bytes_per_pixel;
    let pitch = &boot_info.framebuffer.as_ref().unwrap().info().stride;
    let colorinfo = &boot_info.framebuffer.as_ref().unwrap().info().pixel_format;

    let vga_buffer = boot_info.framebuffer.as_mut().unwrap().buffer_mut();

    let mut color = colorToU32(0, 0, 255, colorinfo);

    for i in 0.. *width {
        for j in 0.. *height {
            putpixel(vga_buffer, i, j, color, *pixelheight, *pitch);
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

fn putpixel (framebuffer: &mut [u8], x: usize, y: usize, color: u32, pixelheight: usize, pitch: usize) {
    let offset = (x * pixelheight) + (y * pitch * pixelheight);
    framebuffer[offset] = (color & 255) as u8;
    framebuffer[offset + 1] = ((color >> 8) & 255) as u8;
    framebuffer[offset + 2] = ((color >> 16) & 255) as u8;
}

fn colorToU32 (r: u32, g: u32, b: u32, colorinfo: &bootloader_api::info::PixelFormat) -> u32 {
    let mut color32 = 0;
    match colorinfo {
        bootloader_api::info::PixelFormat::Rgb => {
            color32 = (b << 16) | (g << 8) | r;
        },
        bootloader_api::info::PixelFormat::Bgr => {
            color32 = (r << 16) | (g << 8) | b;
        },
        bootloader_api::info::PixelFormat::U8 => {
            color32 = r;
        },
        bootloader_api::info::PixelFormat::Unknown { red_position, green_position, blue_position } => {
            color32 = (r << red_position) | (g << green_position) | (b << blue_position);
        },
        _ => {
            color32 = 0;
        },
    }
    color32
}