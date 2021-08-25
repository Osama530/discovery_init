#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f3xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
      let peripherals = pac::Peripherals::take().unwrap(); //initializing all the peripherals (external circuitry)

      let mut rcc = peripherals.RCC.constrain();
      let mut gpioe = peripherals.GPIOE.split(&mut rcc.ahb);

      let mut led_1 = gpioe
            .pe13
            .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

      let mut led_2 = gpioe
            .pe8
            .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

      loop {
            led_1.toggle().unwrap();
            asm::delay(8_000_000);
            led_2.toggle().unwrap();
            asm::delay(6_000_000);
      }
}