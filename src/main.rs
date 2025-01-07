#![deny(unsafe_code)]
#![no_main]
#![no_std]

// Print panic message to probe console
use panic_probe as _;

use cortex_m_rt::entry;
use stm32f0xx_hal::{pac, prelude::*};

use defmt_rtt as _;

use panic_probe as _;

#[allow(clippy::empty_loop)]
#[entry]
fn main() -> ! {
    if let Some(mut p) = pac::Peripherals::take() {
        let mut rcc = p.RCC.configure().sysclk(8.mhz()).freeze(&mut p.FLASH);

        let gpioa = p.GPIOA.split(&mut rcc);

        // (Re-)configure PA1 as output
        let mut led = cortex_m::interrupt::free(|cs| gpioa.pa4.into_push_pull_output(cs));

        loop {
            // Turn PA1 on a million times in a row
            for _ in 0..1_000 {
                led.set_high().ok();
            }
            // Then turn PA1 off a million times in a row
            for _ in 0..1_000 {
                led.set_low().ok();
            }
        }
    }

    loop {
        continue;
    }
}
