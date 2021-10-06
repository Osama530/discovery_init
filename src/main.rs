#![no_std]
#![no_main]

extern crate cortex_m_semihosting;
#[macro_use]
extern crate lazy_static;

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f3::{stm32f303, stm32f303::interrupt};
use cortex_m_semihosting::hprintln;

use cortex_m::{interrupt::Mutex, peripheral::NVIC};
use core::cell::RefCell;
use lazy_static::lazy_static;

// we will need to declare and initialize new global instances for each shared peripheral:
lazy_static!{
      static ref MUTEX_GPIOA: Mutex<RefCell<Option<stm32f303::GPIOA>>> = Mutex::new(RefCell::new(None));
      static ref MUTEX_EXTI: Mutex<RefCell<Option<stm32f303::EXTI>>> = Mutex::new(RefCell::new(None));
      // static ref COUNTS: Mutex<RefCell<interrupt_count>> = Mutex::new(RefCell::new(None));
}
static mut counter: u32 = 0;

#[entry]
fn main() -> ! {
      let peripherals = stm32f303::Peripherals::take().unwrap();
      let rcc = &peripherals.RCC;
      let sys_cfg = &peripherals.SYSCFG;
      let exti = &peripherals.EXTI;
      let gpioa = &peripherals.GPIOA;
      let gpioe = &peripherals.GPIOE;

// enabling the gpioe and gpioa and SYS_CFG registers
      rcc.ahbenr.write(|w| w
            .iopaen().set_bit()
            .iopeen().set_bit()
      );

      rcc.apb2enr.write(|w| w
            .syscfgen().set_bit()
      );


// configuring pin input output configuration
      gpioe.moder.write(|w| w
            .moder8().bits(01)
            .moder9().bits(01)
      );

      gpioa.moder.write(|w| w
            .moder0().bits(00)
      );
      
      gpioa.pupdr.write(|w| unsafe {
      w
            .pupdr0().bits(10)
      });
// We want to generate an interrupt on the EXTI0 line in response to pin PA0 (connecting PA0-->exti0)
      sys_cfg.exticr1.write(|w| unsafe{
      w
            .exti0().bits(0b000)
      });
// Configuring the exti line
      exti.imr1.write(|w|
      w
            .mr0().set_bit() //unmasking interrupt line 0
      );

      exti.rtsr1.write(|w|
      w
            .tr0().set_bit() //rising triger adge selection for exti 0
      );

      // let mut interrupt_count;
// move the GPIOA and EXTI peripherals into the Mutex:
// After this we can only access them via their respective mutex
      cortex_m::interrupt::free(|cs|{
            MUTEX_GPIOA.borrow(cs).replace(Some(peripherals.GPIOA));
            MUTEX_EXTI.borrow(cs).replace(Some(peripherals.EXTI));
            // COUNTS.borrow(cs).replace(Some(interrupt_count));
      });

// Finally you can enable interrupts on the EXTI0 line and enter the main loop:
      // nvic.enable(stm32f303::Interrupt::EXTI0); //depricated call
      unsafe{ NVIC::unmask(stm32f303::Interrupt::EXTI0) };

      loop{
            
      }
}
#[interrupt]
fn EXTI0() {
      let mut interrupt_count = 0;
      cortex_m::interrupt::free (|cs| {
            let exti = MUTEX_EXTI.borrow(cs).borrow();
            exti.as_ref().unwrap().pr1.write(|w|
                  w
                  .pr0().set_bit());
      });
      let button_state = cortex_m::interrupt::free(|cs|{
            let gpio_a = MUTEX_GPIOA.borrow(cs).borrow();
            gpio_a.as_ref().unwrap().idr.read().idr0().bit_is_set()
      });
      if button_state {
            unsafe { counter += 1; 
            hprintln!("intrrupt awakes = {}",counter); };
      }
}


// lazy_stats : code to be executed at runtime in order to be initialized. This includes anything requiring heap allocations, like vectors or hash maps, as well as anything that requires non-const function calls to be computed.
// mutex : share data between threads without running into race condition
// Refcell : for achiving interior mutability