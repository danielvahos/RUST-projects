#![no_std]
#![no_main]
//use rtt_target::rtt_init_print;
//use rtt_target::rprintln;

use cortex_m_rt::entry;
use stm32l4 as _;   // Just to link it in the executable (it provides the vector table)

use panic_probe as _;
use defmt_rtt as _;



/*
#[panic_handler]
fn panic_handler(_panic_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
*/
//use panic_rtt_target as _;

#[entry]
fn main() -> ! {
    //rtt_init_print!();
    defmt::info!("Hello world!");
    //rprintln!("Hello, world!");
    panic!("The program stopped");
 }
