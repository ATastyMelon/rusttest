#![no_std]
#![no_main]

use core::panic::PanicInfo;

use cortex_m_semihosting::hprintln;

use embedded_hal::digital::OutputPin;
use hal::entry;

use rp2040_hal::{self as hal, pac::watchdog};

#[unsafe(link_section = ".boot2")]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

const XTAL_FREQ_HZ: u32 = 12_000_000u32;

#[entry]
fn main() -> ! {
    let mut pac = hal::pac::Peripherals::take().unwrap();
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog
    )
    .ok()
    .unwrap();

    let mut sio = hal::Sio::new(pac.SIO);

    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS   
    );

    let mut led_pin = pins.gpio25.into_push_pull_output();

    led_pin.set_high().unwrap();

    loop {}
    
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    hprintln!("{}", _info);
    loop {}
}