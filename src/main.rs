#![no_std]
#![no_main]

extern crate cortex_m_semihosting;

use cortex_m::asm;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f3xx_hal::{pac, prelude::*, serial:: {Serial, Event}};
use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {

      let mut peripherals = pac::Peripherals::take().unwrap();
      let mut flash = peripherals.FLASH.constrain();
      let mut rcc = peripherals.RCC.constrain();
      let mut gpioa = peripherals.GPIOA.split(&mut rcc.ahb);
      let usart1 = peripherals.USART1;
      
      //initializing the clock for usart as system clock
      let clock = rcc.cfgr.sysclk(48.MHz()).freeze(&mut flash.acr);
      
      //initialing usart pins as alternate fuction high
      let mut tx = gpioa.pa9.into_af7_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);  
      let mut rx=  gpioa.pa10.into_af7_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);
      
      // setting baud rate and clock source for usart1
      let mut serial = Serial::new(
            usart1,
            (tx,rx),
            115200.Bd(),
            clock,
            &mut rcc.apb2,
      );
      
      
      
      loop {
            // while !usart1.isr.read().rxne().bit_is_set()
            if serial.is_event_triggered(Event::ReceiveDataRegisterNotEmpty) {
                  let data = serial.read().unwrap();
                  serial.write(u8::from(b'z')).unwrap();
                  hprintln!("data = {}",data);
                  
            }; 
            
            for byte in b"hay whats up dude".iter() {
                  // wait until it's safe to write to TDR
                  serial.write(u8::from(*byte)).unwrap();
                  while !serial.is_event_triggered(Event::TransmissionComplete) {}
                  // print!("{}", );
            }


      }

}

// wiring 
// Arduino     Ftdi     Stm32
// ACM1        USB0      ACM0
// TX           RX        RX
// Rx           TX        TX
