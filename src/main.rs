#![no_std]
#![no_main]

use core::panic::PanicInfo;

use embassy_executor::Spawner;
use embassy_rp::{self as hal, gpio::{Level, Output}, watchdog::Watchdog};
use embassy_time::Timer;

use defmt_rtt as _;

#[allow(unused)]
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = hal::init(Default::default());
    let mut watchdog = Watchdog::new(p.WATCHDOG);
    let mut led_pin = Output::new(p.PIN_25, Level::Low);

    led_pin.set_high();

    loop {
        Timer::after_millis(15).await;
        watchdog.feed();
    }
    
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}