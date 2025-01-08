#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Pull, Speed};
use embassy_stm32::time::{hz, khz, mhz};
use embassy_stm32::timer::input_capture::{CapturePin, InputCapture};
use embassy_stm32::timer::{self, Channel};
use embassy_stm32::{bind_interrupts, peripherals};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

/// Connect PA2 and PC13 with a 1k Ohm resistor

#[embassy_executor::task]
async fn blinky(led: peripherals::PA4) {
    let mut led = Output::new(led, Level::High, Speed::Low);

    loop {
        //info!("high");
        led.set_high();
        Timer::after_millis(300).await;

        //info!("low");
        led.set_low();
        Timer::after_millis(300).await;
    }
}

bind_interrupts!(struct Irqs {
    TIM1_CC => timer::CaptureCompareInterruptHandler<peripherals::TIM1>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    unwrap!(spawner.spawn(blinky(p.PA4)));

    let ch2: CapturePin<'_, peripherals::TIM1, timer::input_capture::Ch2> =
        CapturePin::new_ch2(p.PA9, Pull::None);

    let mut ic: InputCapture<'_, peripherals::TIM1> = InputCapture::new(
        p.TIM1,
        None,
        Some(ch2),
        None,
        None,
        Irqs,
        hz(1000),
        Default::default(),
    );

    let mut prev_counter = 0;

    loop {
        ic.wait_for_any_edge(Channel::Ch2).await;

        let current_counter = ic.get_capture_value(Channel::Ch2);

        let diff;

        if prev_counter > current_counter {
            diff = current_counter + 0xFFFF as u32 - prev_counter;
        } else {
            diff = current_counter - prev_counter;
        }

        info!("new capture! {}, diff {}", current_counter, diff);

        prev_counter = current_counter;
    }
}

// 38188
