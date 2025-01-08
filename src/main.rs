#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_probe as _;

use stm32f0xx_hal::{self as hal};
use timer::setup_1ns_timer;

use cortex_m_rt::entry;

mod clock;
use clock::setup_clock;
mod timer;

#[entry]
fn main() -> ! {
    defmt::info!("Hello, world!");
    setup_clock();
    defmt::info!("Clock setup done");

    setup_1ns_timer();
    defmt::info!("timer setup done");

    loop {
        cortex_m::asm::wfi();
    }
}
