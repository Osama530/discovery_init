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
            .moder15().bits(01)
      );

      gpioa.moder.write(|w| w
            .moder0().bits(00)
      );
      
      gpioa.pupdr.write(|w| unsafe {
      w
            .pupdr0().bits(00)
      });


let mut count = 0;

loop {

      let button_state = gpioa.idr.read().idr0();

      if button_state == true {
      count += 1;
      gpioe.bsrr.write(|w| w
            .bs15().set_bit());
      hprintln!("count = {:?}", count);
      asm::delay(6_000_000);  }
     
      else {
            gpioe.bsrr.write(|w| w
                  .br15().set_bit());
            //asm::delay(6_000_000);
      }

   }
}