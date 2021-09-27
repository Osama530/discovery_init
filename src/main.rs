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

      let mut peripherals = &stm32f303::Peripherals::take().unwrap();
      let mut rcc = &peripherals.RCC;
      let gpioa = &peripherals.GPIOA;
      let usart1 = &peripherals.USART1;

//selecting and enabling clock for usart1
      rcc.cfgr3.write(|w| w.usart1sw().bits(01)); //slecting clock source as pc_)clock (deefault)
      rcc.apb2enr.write(|w| w.usart1en().set_bit());
//enabling port and pins assign for usart1
      rcc.ahbenr.write(|w| w.iopaen().set_bit());
   
      gpioa.moder.write(|w| w.moder9().bits(10));     // 00 => input, 01 => output, 10 => Alternate function
      gpioa.otyper.write(|w| w.ot9().clear_bit());          //pa10 = rx,  pa9 = tx
      gpioa.ospeedr.write(|w| unsafe {w.ospeedr9().bits(01)});
      
      gpioa.moder.write(|w| w.moder10().bits(10));
      gpioa.otyper.write(|w| w.ot10().clear_bit());
      gpioa.ospeedr.write(|w| unsafe {w.ospeedr10().bits(01)});
   
      gpioa.afrh.write(|w| w.afrh9().af7());    //selecting corrosponding alternate function register
      gpioa.afrh.write(|w| w.afrh10().af7());

      gpioa.pupdr.write(|w| unsafe {w.pupdr9().bits(00)});
      gpioa.pupdr.write(|w| unsafe {w.pupdr10().bits(00)});
// defining the word length
      usart1.cr1.write(|w| w.m().bit8());
// seting baud rate for usart1
      usart1.cr1.write(|w| w.over8().set_bit());
      usart1.brr.write(|w| w.brr().bits(0x47));
// setting numbers of stop bits
      usart1.cr2.write(|w| w.stop().bits(00));
// enabling usart1 and transmission
      usart1.cr1.write(|w|
            w
            .ue().set_bit()
            .te().set_bit()
            .re().set_bit());
// send data
while usart1.isr.read().txe().bit_is_clear() {}
      // Send a string
    for byte in b"The quick brown fox jumps over the lazy dog.".iter() {
      usart1
          .tdr
          .write(|w| w.tdr().bits(u16::from(*byte)));
  }
// wait till transmit complete and tdr register empty

//receive data
      loop {

      while usart1.isr.read().txe().bit_is_clear() {}
            // Send a string
          for byte in b"The quick brown fox jumps over the lazy dog.".iter() {
            usart1
                .tdr
                .write(|w| w.tdr().bits(u16::from(*byte)));
        }
      
      while !usart1.isr.read().rxne().bit_is_set() {}
      let data = usart1.rdr.read().rdr().bits();
      hprintln!("data recived = {}",data);
      }
}