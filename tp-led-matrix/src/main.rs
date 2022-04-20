#![no_std]
#![no_main]
//use rtt_target::rtt_init_print;
//use rtt_target::rprintln;

//use cortex_m_rt::entry;
//use stm32l4 as _;   // Just to link it in the executable (it provides the vector table)

use panic_probe as _;
use defmt_rtt as _;

use stm32l4xx_hal::{gpio::*,pac, prelude::*};


use tp_led_matrix::image::{Color, Image, RED, GREEN, BLUE};
use tp_led_matrix::matrix::Matrix;
use tp_led_matrix::image::Default;


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






//USARTS 
#[rtic::app(device = stm32l4xx_hal::pac, dispatchers = [USART2, USART3])]
//#[rtic::app(device = stm32l4xx_hal::pac)]
mod app {
    use super::*; //to be able to access Color, Image and so on from RTIC app


    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        matrix: Matrix
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        defmt::info!("defmt correctly initialized");
    
        let _cp = cx.core;
        let dp = cx.device;
    
        // Initialize the clocks, hardware and matrix using your existing code

        let mut rcc = dp.RCC.constrain();
        let mut flash = dp.FLASH.constrain();
        let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);
    

        // Setup the clocks at 80MHz using HSI (by default since HSE/MSI are not configured).
        // The flash wait states will be configured accordingly.
        let clocks = rcc.cfgr.sysclk(80.MHz()).freeze(&mut flash.acr, &mut pwr);
    
        //Create Image gradient Blue
        let newimage=Image::gradient(BLUE);
        let mut gpioa= dp.GPIOA.split(&mut rcc.ahb2);
        let mut gpiob= dp.GPIOB.split(&mut rcc.ahb2);
        let mut gpioc= dp.GPIOC.split(&mut rcc.ahb2);

        let mut gpioa_moder=gpioa.moder;
        let mut gpioa_otyper= gpioa.otyper;
        let mut gpiob_moder= gpiob.moder;
        let mut gpiob_otyper= gpiob.otyper;
        let mut gpioc_moder= gpioc.moder;
        let mut gpioc_otyper= gpioc.otyper;

            //Create Matrix
        let mut matrix= Matrix::new(gpioa.pa2,gpioa.pa3,gpioa.pa4,gpioa.pa5,gpioa.pa6,gpioa.pa7,gpioa.pa15,
                        gpiob.pb0,gpiob.pb1,gpiob.pb2,
                        gpioc.pc3,gpioc.pc4,gpioc.pc5,
                    &mut gpioa_moder,&mut gpioa_otyper,
                    &mut gpiob_moder,&mut gpiob_otyper,
                    &mut gpioc_moder,&mut gpioc_otyper,
                clocks,);


        // Return the resources and the monotonic timer
        (Shared {}, Local { matrix }, init::Monotonics())
    }



    #[idle(local = [matrix])]
    fn idle(cx: idle::Context) -> ! {
        // Display an image on the LED matrix in an infinite loop
        let newimage=Image::gradient(BLUE);
        loop{
            cx.local.matrix.display_image(&newimage);
        }
    }


    /*
    #[entry]

    fn main() -> ! {
        let cp = pac::CorePeripherals::take().unwrap();
        let dp = pac::Peripherals::take().unwrap();

        run(cp, dp)
    }
     */

    /* 

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

    */


}
