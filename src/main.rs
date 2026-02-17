#![no_std]
#![no_main]

use core::panic::PanicInfo;

use cortex_m_semihosting::hprintln;

use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;

use hal::entry;

use rp2040_hal as hal;

unsafe extern "C" {
    static __PAYLOAD: u32;
    static __PAYLOAD_END: u32;
}

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
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let sio = hal::Sio::new(pac.SIO);

    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);
    
    let mut led_pin = pins.gpio25.into_push_pull_output();

    loop {
        led_pin.set_high().unwrap();
        timer.delay_ms(500);
        led_pin.set_low().unwrap();
        timer.delay_ms(500);

        watchdog.feed();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    hprintln!("{}", _info);
    loop {}
}