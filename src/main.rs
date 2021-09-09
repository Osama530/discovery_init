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
      gpioa.moder.write(|w| w.moder10().bits(10));
   
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
// enableing usart1 and transmission
      usart1.cr1.write(|w|
            w
            .ue().set_bit()
            .te().set_bit());
// wait till transmit data register empty
      while !usart1.isr.read().txe().bit_is_set() {}
// send data
      usart1.tdr.write(|w| w.tdr().bits(u16::from(b'X')));

      
      
      loop {
      while usart1.isr.read().tc().bit_is_clear() {}
      let txe_status = usart1.isr.read().txe().bits(); 
      let value = usart1.tdr.read().tdr().bits() as u8;
      hprintln!("tdr value = {} and txe status = {}", value, txe_status);
}
}