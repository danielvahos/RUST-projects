#![no_std]
#![no_main]
//use rtt_target::rtt_init_print;
//use rtt_target::rprintln;

use cortex_m_rt::entry;
//use stm32l4 as _;   // Just to link it in the executable (it provides the vector table)

use panic_probe as _;
use defmt_rtt as _;

use stm32l4xx_hal::{pac, prelude::*};


/*
#[panic_handler]
fn panic_handler(_panic_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
*/
//use panic_rtt_target as _;
/* 
#[entry]
fn main() -> ! {
    //rtt_init_print!();
    defmt::info!("Hello world!");
    //rprintln!("Hello, world!");
    panic!("The program stopped");
 }
*/

 #[entry]
fn main() -> ! {
    let cp = pac::CorePeripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    run(cp, dp)
}

fn run(_cp: pac::CorePeripherals, dp: pac::Peripherals) -> ! {
    // Get high-level representations of hardware modules
    let mut rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);

    // Setup the clocks at 80MHz using HSI (by default since HSE/MSI are not configured).
    // The flash wait states will be configured accordingly.
    let clocks = rcc.cfgr.sysclk(80.MHz()).freeze(&mut flash.acr, &mut pwr);

    defmt::info!("Hello, world!");
    panic!("The program stopped");
}
