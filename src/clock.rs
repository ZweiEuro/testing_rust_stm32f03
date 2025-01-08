use panic_probe as _;

use crate::hal::{pac::Peripherals, prelude::*};
use cortex_m::peripheral::Peripherals as c_m_Peripherals;

pub fn setup_clock() -> () {
    if let (Some(mut p), Some(_cp)) = (Peripherals::take(), c_m_Peripherals::take()) {
        p.RCC
            .configure()
            .sysclk(48.mhz())
            .pclk(24.mhz())
            .freeze(&mut p.FLASH);
    } else {
        panic!("Error: Could not config RCC");
    }
}
