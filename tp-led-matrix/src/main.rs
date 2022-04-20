#![no_std]
#![no_main]
//use rtt_target::rtt_init_print;
//use rtt_target::rprintln;

use cortex_m_rt::entry;
//use stm32l4 as _;   // Just to link it in the executable (it provides the vector table)

use panic_probe as _;
use defmt_rtt as _;

use stm32l4xx_hal::{pac, prelude::*};


use tp_led_matrix::image::{Color, Image, RED, GREEN, BLUE};
use tp_led_matrix::matrix::Matrix;


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



    let newimage=Image::gradient(BLUE);
    let gpioa= dp.GPIOA.split(&mut rcc.ahb2);
    let gpiob= dp.GPIOB.split(&mut rcc.ahb2);
    let gpioc= dp.GPIOC.split(&mut rcc.ahb2);

    let mut newMatrix=Matrix::new(gpioa.pa2,gpia.pa3,gpioa.pa4,gpioa.pa5,gpioa.pa6,gpioa.pa7,gpioa.pa15, gpiob.pb0, gpiob.pb1,gpiob.pb2, gpioc.pc3, gpioc.pc4,gpioc.pc5, &mut gpioa.moder,&mut gpioa.otyper,  &mut gpiob.moder,&mut gpiob.otyper ,  &mut gpioc.moder,&mut gpioc.otyper );

    defmt::info!("Hello, world!");
    loop{
        newMatrix.display_image(&newImage);
    }
    //panic!("The program stopped");
}