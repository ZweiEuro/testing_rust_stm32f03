#![no_std]
#![no_main]

use core::cell::RefCell;

use cortex_m::interrupt::{self, Mutex};
use cortex_m_rt::entry;
use defmt::*;
use embassy_stm32::{
    gpio::{Level, Output, Speed},
    peripherals,
};
use {defmt_rtt as _, panic_probe as _};

static ADV_TIMER: Mutex<RefCell<Option<embassy_stm32::peripherals::TIM1>>> =
    Mutex::new(RefCell::new(None));

fn init_timer() {}

fn toggle_led(led: peripherals::PA4) {
    let mut led = Output::new(led, Level::High, Speed::Low);
    led.toggle();
}

#[entry]
fn main() -> ! {
    let p = embassy_stm32::init(Default::default());

    interrupt::free(|cs| ADV_TIMER.borrow(cs).replace(Some(p.TIM1)));

    init_timer();

    toggle_led(p.PA4);

    info!("Hello, world!");

    loop {}
}

// 38188
