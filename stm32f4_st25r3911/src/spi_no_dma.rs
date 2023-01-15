#![no_main]
#![no_std]

use cortex_m::asm::delay;
//use cortex_m::delay;
// Halt on panic
use panic_halt as _;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use embedded_hal::spi::{Mode, Phase, Polarity};
use stm32f4xx_hal::pac::interrupt;
use stm32f4xx_hal::{
    dma::{config, traits::StreamISR, MemoryToPeripheral, Stream2, StreamsTuple, Transfer},
    pac,
    prelude::*,
    spi::*,
};
use cortex_m_semihosting::hprintln;

const ARRAY_SIZE: usize = 2;

type SpiDma = Transfer<
    Stream2<pac::DMA2>,
    2,
    Tx<pac::SPI1>,
    MemoryToPeripheral,
    &'static mut [u8; ARRAY_SIZE],
>;

static G_TRANSFER: Mutex<RefCell<Option<SpiDma>>> = Mutex::new(RefCell::new(None));

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

        let steams = StreamsTuple::new(dp.DMA2);
        let stream = steams.2;

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
        let rx_buffer = cortex_m::singleton!(: [u8; ARRAY_SIZE] = [1; ARRAY_SIZE]).unwrap();
        /*
        Attempting to read ID register. 0x3F--read mode 'mosi --0b01xxxxxx'
         */
        tx_buffer[0] = 0x40 | 0x3F;
        tx_buffer[1] = 0x00;
       

        //hprintln!("{:#x?}", buffer[0]);

        /*for (i, b) in buffer.iter_mut().enumerate() {
            *b = i as u8;
        }*/
        

        let tx = spi.use_dma().tx();
        let rx = spi.use_dma().rx();

        //let result = spi.transfer(buffer);
       
        /* 
        hprintln!("{:#x?}", buffer[0]);
        hprintln!("{:#x?}", buffer[1]);
        hprintln!("{:#x?}", buffer[2]);
        */
        

         let mut transfer_out = Transfer::init_memory_to_peripheral(
            stream,
            tx,
            tx_buffer,
            None,
            config::DmaConfig::default()
                .memory_increment(true)
                .fifo_enable(true)
                .fifo_error_interrupt(true)
                .transfer_complete_interrupt(false),
        );

        let mut transfer_in = Transfer::init_peripheral_to_memory(
            stream,
            rx,
            rx_buffer,
            None,
            config::DmaConfig::default()
            .memory_increment(true)
            .fifo_enable(true)
            .fifo_error_interrupt(true)
            .transfer_complete_interrupt(false),
        );

        cs.set_low();
        delay(1000);

        let result = transfer_out.start(|_tx| {});
        

        delay(1000);
        cs.set_high();
        hprintln!("{:?}", result);
        

        // Hand off transfer to interrupt handler
        cortex_m::interrupt::free(|cs| *G_TRANSFER.borrow(cs).borrow_mut() = Some(transfer));
        // Enable interrupt
        //hprintln!("{:?}", transfer);
        hprintln!("{:?}", result);
        unsafe {
            cortex_m::peripheral::NVIC::unmask(pac::Interrupt::DMA2_STREAM4);
        }
    }
       loop {
        cortex_m::asm::nop();
    }
    
}

#[interrupt]
fn DMA2_STREAM4() {
    static mut TRANSFER: Option<SpiDma> = None;

    let transfer = TRANSFER.get_or_insert_with(|| {
        cortex_m::interrupt::free(|cs| G_TRANSFER.borrow(cs).replace(None).unwrap())
    });

    // Its important to clear fifo errors as the transfer is paused until it is cleared
    if Stream2::<pac::DMA2>::get_fifo_error_flag() {
        transfer.clear_fifo_error_interrupt();
    }
    if Stream2::<pac::DMA2>::get_transfer_complete_flag() {
        transfer.clear_transfer_complete_interrupt();
        unsafe {
            static mut BUFFER: [u8; ARRAY_SIZE] = [0; ARRAY_SIZE];
            for (i, b) in BUFFER.iter_mut().enumerate() {
                *b = (i + 1) as u8;
            }
            transfer.next_transfer(&mut BUFFER).unwrap();
        }
    }
}
