// Demonstrate the use of a blocking `Delay` using the SYST (sysclock) timer.
//cargo flash --chip  STM32F410RBTx --release

#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _; // panic handler

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the LED. On the Nucleo-446RE it's connected to pin PA5.
        let gpioa = dp.GPIOA.split();
        let mut led = gpioa.pa5.into_push_pull_output();
        let mut led_RFID = gpioa.pa8.into_push_pull_output();

        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

        // Create a delay abstraction based on SysTick
        let mut delay = cp.SYST.delay(&clocks);
        let mut test = 0;

        loop {
            // On for 1s, off for 1s.
         led.set_high();
         led_RFID.set_low();
         delay.delay_ms(1000_u32);
         led.set_low();
         led_RFID.set_high();
         delay.delay_ms(1000_u32);
         test = test + 1;
        }
    }

    loop {}
}