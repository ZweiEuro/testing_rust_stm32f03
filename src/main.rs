#![deny(unsafe_code)]
#![no_main]
#![no_std]

// Print panic message to probe console
use panic_probe as _;

use cortex_m_rt::entry;
use stm32f0xx_hal::{pac, prelude::*};

use defmt_rtt as _;

#[allow(clippy::empty_loop)]
#[entry]
fn main() -> ! {
    if let Some(mut p) = pac::Peripherals::take() {
        let mut rcc = p.RCC.configure().sysclk(8.mhz()).freeze(&mut p.FLASH);

        let gpioa = p.GPIOA.split(&mut rcc);

        // (Re-)configure PA1 as output
        let mut led = cortex_m::interrupt::free(|cs| gpioa.pa4.into_push_pull_output(cs));

        loop {
            led.set_high().ok();
            cortex_m::asm::delay(8_000_000);
            led.set_low().ok();
            cortex_m::asm::delay(8_000_000);
        }
    }

    loop {
        continue;
    }
}
