#![no_std]
#![no_main]

use core::panic::PanicInfo;

use rp2040_hal as hal;

use hal::entry;
use cortex_m_semihosting::hprintln;

#[unsafe(link_section = ".boot2")]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

#[entry]
fn _start() -> ! {
    hprintln!("Hello World!");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}