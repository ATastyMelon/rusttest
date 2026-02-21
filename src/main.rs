#![no_std]
#![no_main]

use core::panic::PanicInfo;

use cortex_m_semihosting::hprintln;

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

    let count_max= 1000;
    let mut count = 0;
    let mut countdown: bool = true;

    loop {

        led_pin.set_high();
        Timer::after_micros(count).await;
        led_pin.set_low();
        Timer::after_micros(count_max - count).await;

        if countdown {
            if count == 0 {
                countdown = false;
            } else {
                count -= 1;
            }
        } else {
            if count < count_max {
                count += 1;
            } else {
                countdown = true;
            }
        }

        watchdog.feed();
    }
    
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    hprintln!("{}", _info);
    loop {}
}