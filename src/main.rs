#![no_std]
#![no_main]

extern crate cortex_m_semihosting;
#[macro_use]
extern crate lazy_static;

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f3::{stm32f303, stm32f303::interrupt};
use cortex_m_semihosting::hprintln;

use cortex_m::{asm,interrupt::Mutex, peripheral::NVIC};
use core::cell::RefCell;
use lazy_static::lazy_static;

// initializing globle variables
const DMA_LENGHT: usize = 64;
static mut DMA_BUFFER: [u32; DMA_LENGHT] =[0; DMA_LENGHT];   

// we will need to declare and initialize new global instances for each shared peripheral:
lazy_static!{
     static ref MUTEX_DMA2: Mutex<RefCell<Option<stm32f303::DMA2>>> = Mutex::new(RefCell::new(None));
}

// initilizing and congigring tim2 timer
pub fn init_tim2(dp: &stm32f303::Peripherals){
      let rcc = &dp.RCC;
      // enabeling tim2 clock
      rcc.apb1enr.write(|w| w.tim2en().set_bit());
      // calculation for tim2 frecuency
      let sys_clock = 8_000_000;    // the stmf32f3 discovery board CPU runs at 8Mhz by default
      let sampling_frequency = 44_100; // we want an audio sampling rate of 44.1KHz
      let arr = sys_clock/sampling_frequency;
      let tim2 = &dp.TIM2;
      tim2.cr2.write(|w| w.mms().update());     // update when counter reaches arr value
      tim2.arr.write(|w| w.arr().bits(arr));    //set timer period (sysclk / fs)
      // finally enabling the timer
      tim2.cr1.write(|w| w.cen().set_bit());
}
// initilizing and congigring GPIO and DAC
pub fn init_gpio(dp: &stm32f303::Peripherals){
      let rcc = &dp.RCC;
      // enebling clocks for gpioa and dac
      rcc.ahbenr.write(|w| w.iopaen().enabled());
      rcc.apb1enr.write(|w| 
      w
            .dac1en().set_bit()
            // .dac2en().set_bit()
      );
      // configuring gpioa as analog floating
      let gpio_a = &dp.GPIOA;
      gpio_a.moder.modify(|_, w|
      w
            .moder4().analog()
            .moder5().analog());
      gpio_a.pupdr.modify(|_, w|
      w
            .pupdr4().floating()
            .pupdr5().floating());
      // now configuring dac
      let dac = &dp.DAC1;
      dac.cr.write(|w|
      w
            .boff1().disabled()     // disable dac output buffer for channel 1
            .boff2().disabled()     // disable dac output buffer for channel 2
            .ten1().enabled()       // enable trigger for channel 1
            .ten2().enabled()       // enable trigger for channel 2
            .tsel1().tim2_trgo()    // set trigger for channel 1 to TIM2
            .tsel2().bits(100)      // set trigger for channel 2 to TIM2
      );
      // finally enabling the dac
      dac.cr.modify(|_,w|
      w
            .en1().set_bit()
            .en2().set_bit());
}

pub fn init_dma(cp:&cortex_m::Peripherals, dp: &stm32f303::Peripherals){
      // enableing dma clock
      let rcc = &dp.RCC;
      rcc.ahbenr.modify(|_, w| w.dma1en().enabled());
      // the memory address of the data (source) to be transferred
      let ma = unsafe{
            DMA_BUFFER.as_ptr()
      } as usize as u32;
      // destination memory address
      let pa = 0x40007420;    // destination: Dual DAC 12-bit right-aligned data holding register (DHR12RD)
      let ndt = DMA_LENGHT as u16;  // number of items to transfer
      // configure and enable DMA2 channel 3
      let dma2 = &dp.DMA2;
      dma2.ch3.mar.write(|w| unsafe{ w.ma().bits(ma) });    // source memory address
      dma2.ch3.par.write(|w| unsafe{ w.pa().bits(pa) });// destination peripheral address
      dma2.ch3.ndtr.write(|w| unsafe{ w.ndt().bits(ndt) });

      // chennel configutration settings
      dma2.ch3.cr.write(|w| {
            w.dir().from_memory()   // source is memory
             .mem2mem().disabled()  // disable memory to memory transfer
             .minc().enabled()      // increment memory address every transfer
             .pinc().disabled()     // don't increment peripheral address every transfer
             .msize().bits32()       // memory word size is 32 bits
             .psize().bits32()       // peripheral word size is 32 bits
             .circ().enabled()      // dma mode is circular
             .pl().high()           // set dma priority to high
             .teie().enabled()      // trigger an interrupt if an error occurs
             .tcie().enabled()      // trigger an interrupt when transfer is complete
             .htie().enabled()      // trigger an interrupt when half the transfer is complete
        });

      // enable DMA interrupt
      let nvic = &mut cp.NVIC;
      nvic.enable(stm32f303::Interrupt::DMA2_CH3);
}


#[entry]
fn main() -> ! {


      loop{
         
            
      }
}


// lazy_stats : code to be executed at runtime in order to be initialized. This includes anything requiring heap allocations, like vectors or hash maps, as well as anything that requires non-const function calls to be computed.
// mutex : share data between threads without running into race condition
// Refcell : for achiving interior mutability