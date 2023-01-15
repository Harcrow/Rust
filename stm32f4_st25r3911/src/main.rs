#![no_main]
#![no_std]

use cortex_m::asm::delay;
//use cortex_m::delay;
// Halt on panic
use panic_halt as _;
use cortex_m_rt::entry;
use embedded_hal::spi::{Mode, Phase, Polarity};

use stm32f4xx_hal::{
    pac,
    prelude::*,
    spi::*,
};
use cortex_m_semihosting::hprintln;

const ARRAY_SIZE: usize = 2;
const TX_ARRAY: usize = 2;

//registers
const READ: u8 = (0 << 7 | 1 << 6);
const WRITE: u8 = (0 << 7 | 0 << 6);

const COMMAND: u8 = (1 << 7 | 1 << 6);

//FIFO
const FIFO_LOAD: u8 = 0x80;
const FIFO_READ: u8 = 0xBF;

#[entry]
fn main() -> ! {
    if let Some(dp) = pac::Peripherals::take() {
        // Set up the system clock.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.freeze();

        let gpioa = dp.GPIOA.split();
        let gpiob = dp.GPIOB.split();

         //cs
        let mut cs = gpiob.pb6.into_push_pull_output();
        cs.set_high();


        //clk, miso, mosi, respectively
        let pa5 = gpioa.pa5.into_alternate().internal_pull_up(true);
        let pa6 = gpioa.pa6.into_alternate();
        let pa7 = gpioa.pa7.into_alternate();


        let mode = Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnSecondTransition,
        };

        let mut spi = Spi::new(dp.SPI1,
                          (pa5, pa6, pa7),
                           mode,
                           1.MHz(),
                           &clocks);

        let tx_buffer = cortex_m::singleton!(: [u8; ARRAY_SIZE] = [1; ARRAY_SIZE]).unwrap();
        
        /*
        Attempting to read ID register. 0x3F--read mode 'mosi --0b01xxxxxx'
         */
        tx_buffer[0] = 0x40 | 0x3F;
        tx_buffer[1] = 0x00;
       

        //hprintln!("{:#x?}", buffer[0]);

        /*for (i, b) in buffer.iter_mut().enumerate() {
            *b = i as u8;
        }*/

        cs.set_low();
        delay(1000);


        let result = spi.transfer(tx_buffer);
          
       
        delay(1000);
        cs.set_high();
        hprintln!("{:#x?}", result);
        

        // Hand off transfer to interrupt handler
        
    }
       loop {
        cortex_m::asm::nop();
    }
    
}

