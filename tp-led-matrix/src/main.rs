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

use dwt_systick_monotonic::DwtSystick;
use dwt_systick_monotonic::ExtU32;


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

    #[monotonic(binds = SysTick, default = true)]
    type MyMonotonic = DwtSystick<80_000_000>;
    type Instant = <MyMonotonic as rtic::Monotonic>::Instant;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        matrix: Matrix,
        newimage:Image
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        defmt::info!("defmt correctly initialized");
    
        let mut cp = cx.core;
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

        //Monotonic timer instance creation
        let mut mono = DwtSystick::new(&mut cp.DCB, cp.DWT, cp.SYST, 80_000_000);

        //Build image to display
        let image_1:Image= Image::new_solid(Color{r:0,g:255,b:0});
        //let image_1:Image=image_1.default(); //one use default for the image
        display::spawn(mono.now()).unwrap(); //display task  gets spawned after init() terminates
        
        // Return the resources and the monotonic timer
        (Shared {}, Local { matrix, newimage }, init::Monotonics(mono))
    }



    #[idle(local = [count:usize=0])]
    fn idle(cx: idle::Context) -> ! {

        /* 
        // Display an image on the LED matrix in an infinite loop
        let newimage=Image::gradient(BLUE);
        loop{
            cx.local.matrix.display_image(&newimage);
        }
        */
        loop{
            if *cx.local.count %10_000== 0{
                defmt::info!("HOLA AMIGOS");
                *cx.local.count = 0;
            }
            *cx.local.count = *cx.local.count + 1;
        }
    }

    #[task(local = [matrix, newimage, next_line: usize = 0])]
    fn display(cx: display::Context, at:Instant) {
        // Display line next_line (cx.local.next_line) of
        // the image (cx.local.image) on the matrix (cx.local.matrix).
        // All those are mutable references.
        let next=1.secs()/(6*80);
        let mut next_line = 0;
        //let next_line: &mut usize=cx.local.next_line;

        // Increment next_line up to 7 and wraparound to 0
        /*
        next_line +=1;
        if next_line>7{
            next_line=0;
        }
        display::spawn_at(at, at+next).unwrap();
        */
        for i in 0..8{
            cx.local.matrix.send_row(next_line,cx.local.newimage.row(next_line));
            next_line += 1;
            if next_line == 8{
                next_line = 0;
            }
            }
        display::spawn_at(at, at + next).unwrap();
        /* 
        *cx.local.next_line +=1;
        if *cx.local.next_line>7{
            *cx.local.next_line=0;
        }
        display::spawn_at(at, at+next).unwrap();
        */
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
