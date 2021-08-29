#![no_std]
#![no_main]

extern crate cortex_m_semihosting;

use cortex_m::asm;
use cortex_m_rt::entry;
use panic_halt as _;
//use stm32f3xx_hal::{pac, prelude::*}; without using hall
use stm32f3::stm32f303;
use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
      let mut peripherals = stm32f303::Peripherals::take().unwrap();
      let mut rcc = peripherals.RCC;

      let gpioa = &peripherals.GPIOA;
      let gpioe = &peripherals.GPIOE;

// enabling the gpioe and gpioa registers
      rcc.ahbenr.write(|w| w
            .iopaen().set_bit()
            .iopeen().set_bit()
      );

// configuring pin input output configuration
      gpioe.moder.write(|w| w
            .moder8().bits(01)
            .moder9().bits(01)
            .moder10().bits(01)
            .moder11().bits(01)
            .moder12().bits(01)
            .moder13().bits(01)
            .moder14().bits(01)
            .moder15().bits(01)

      );

      gpioa.moder.write(|w| w
            .moder0().bits(00)
      );
      
      gpioa.pupdr.write(|w| unsafe {
      w
            .pupdr0().bits(00)
      });


let mut count = 7;

loop {
      
      let button_state = gpioa.idr.read().idr0();
      if button_state == true {
      count += 1; }

      match count {
            8 => { gpioe.bsrr.write(|w| w.bs8().set_bit());
                  asm::delay(4_000_000); },
            9 =>{ gpioe.bsrr.write(|w| w.bs9().set_bit());
            asm::delay(4_000_000); },

            10 =>{ gpioe.bsrr.write(|w| w.bs10().set_bit());
            asm::delay(4_000_000); },

            11 =>{ gpioe.bsrr.write(|w| w.bs11().set_bit());
            asm::delay(4_000_000); },

            12 =>{ gpioe.bsrr.write(|w| w.bs12().set_bit());
            asm::delay(4_000_000); },

            13 =>{ gpioe.bsrr.write(|w| w.bs13().set_bit());
            asm::delay(4_000_000); },

            14 =>{ gpioe.bsrr.write(|w| w.bs14().set_bit());
            asm::delay(4_000_000); },

            15 =>{ gpioe.bsrr.write(|w| w.bs15().set_bit());
            asm::delay(4_000_000); },

            _ => { if count >= 15 {
                  count = 7;
                  gpioe.bsrr.write(|w| w
                        .br8().set_bit()
                        .br9().set_bit()
                        .br10().set_bit()
                        .br11().set_bit()
                        .br12().set_bit()
                        .br13().set_bit()
                        .br14().set_bit()
                        .br15().set_bit())
                  
            }
       }
            
            }
      }
}