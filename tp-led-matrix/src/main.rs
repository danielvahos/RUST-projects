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

use stm32l4xx_hal::serial::{Config, Event, Rx, Serial};

//pub use binary_heap::BinaryHeap;	
//pub use pool::singleton::arc::Arc;


use heapless::pool::{Box, Node, Pool};

use core::mem::swap;
use core::mem::MaybeUninit;

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
    struct Shared {
        newimage: Image,
        next_image: Option<Box<Image>>,
        image: Image,
        pool: Pool<Image>

    }

    #[local]
    struct Local {
        matrix: Matrix,
        usart1_rx: Rx<stm32l4xx_hal::stm32::USART1>,
        //next_image: Image,
        current_image:Box<Image>,
        rx_image: Box<Image>
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

        let pb6_alter/* Pin<Alternate<PushPull, 7>, _>*/ =
            gpiob
                .pb6//Pin<Analog, L8, _, 6>
                .into_alternate(&mut gpiob_moder, &mut gpiob_otyper, &mut gpiob.afrl);
        let pb7_alter/* Pin<Alternate<PushPull, 7>, _>*/ =
                gpiob
                .pb7//Pin<Analog, L8, _, 7>
                .into_alternate(&mut gpiob_moder, &mut gpiob_otyper, &mut gpiob.afrl);


        //Set the baudrate
        let bit_psec= stm32l4xx_hal::time::Bps(38400);
        let baudr= stm32l4xx_hal::serial::Config::default().baudrate(bit_psec);

        //Intialization of the serial port
        let mut serialport =stm32l4xx_hal::serial::Serial::usart1(
            dp.USART1,
            (pb6_alter,pb7_alter),
            baudr,
            clocks,
            &mut rcc.apb2,
        );
        serialport.listen(Event::Rxne);//to enable the "RX not empty" event - triigger interr when character received
        let (_tx, usart1_rx)=serialport.split();//get the receiver part of the serial port



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
        let image:Image= Image::new_solid(Color{r:0,g:0,b:0});
        //let next_image:Image= Image::new_solid(Color{r:0,g:0,b:0});
        //let image_1:Image=image_1.default(); //one use default for the image
        //rotate_image::spawn(mono.now(), 0).unwrap();
        display::spawn(mono.now()).unwrap(); //display task  gets spawned after init(ternunate)
        // Return the resources and the monotonic timer




        //Triple buffer
        let pool: Pool<Image> = Pool::new();
        unsafe {
          static mut MEMORY: MaybeUninit<[Node<Image>; 3]> = MaybeUninit::uninit();
          pool.grow_exact(&mut MEMORY);   // static mut access is unsafe
        }
        //let current_image:heapless::pool::Box<Image>;
        //let rx_image:heapless::pool::Box<Image>;
        let current_image = pool.alloc().unwrap().init(Image::default());
        let rx_image = pool.alloc().unwrap().init(Image::default());




        let next_image=None;


      
        (Shared {newimage, image, next_image, pool}, Local {usart1_rx, matrix, current_image, rx_image}, init::Monotonics(mono))
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

    #[task(local = [matrix, next_line: usize = 0], shared = [newimage], priority = 2)]
    fn display(mut cx: display::Context, at:Instant) {
        // Display line next_line (cx.local.next_line) of
        // the image (cx.local.image) on the matrix (cx.local.matrix).
        // All those are mutable references.
        let next=1.secs()/(6*80);
        //let mut next_line = 0;
        let mut next_line= cx.local.next_line;
        let matrix=cx.local.matrix;
        //let next_line: &mut usize=cx.local.next_line;

        // Increment next_line up to 7 and wraparound to 0
        /*
        next_line +=1;
        if next_line>7{
            next_line=0;
        }
        display::spawn_at(at, at+next).unwrap();
        */
        cx.shared.newimage.lock(|newimage| {
            matrix.send_row(*next_line, newimage.row(*next_line));
        });


    
        *next_line = (*next_line+1) %8;
        if *next_line == 0 {
            (cx.shared.newimage, cx.shared.next_image, cx.shared.pool).lock(|newimage,next_image,pool: &mut Pool<Image>| {
                    if let Some(mut next_image) = next_image.take() {
                        swap(&mut next_image, newimage);
                        pool.free(next_image);
                    };
                },
            );
        }

        /* 
        cx.shared.newimage.lock(|newimage| {

        for i in 0..8{
            cx.local.matrix.send_row(next_line,newimage.row(next_line));
            next_line += 1;
            if next_line == 8{
                next_line = 0;
            }
            }
        });
        */
        display::spawn_at(at, at + next).unwrap();
        
        /*
        *cx.local.next_line +=1;
        if *cx.local.next_line>7{
            *cx.local.next_line=0;
        }
        display::spawn_at(at, at+next).unwrap();
        */
    }

    #[task(binds = USART1,
        local = [usart1_rx, next_pos: usize = usize::MAX, current_image],
        shared = [next_image, pool])]
    fn receive_byte(mut cx: receive_byte::Context)
    {
        let next_pos:&mut usize =cx.local.next_pos;
        let image =cx.local.current_image;

        if let Ok(b) = cx.local.usart1_rx.read() {
            if b == 0xFF{
                *next_pos = 0;


            }else if *next_pos != usize::MAX {
                image.as_mut()[*next_pos] = b;


                *next_pos += 1;
            }




             // If the received image is complete, make it available to
            // the display task.
            if *next_pos == 8 * 8 * 3 {
                (cx.shared.next_image, cx.shared.pool).lock(|next_image, pool| {
                    if let Some(next_image) = next_image.take() {
                        pool.free(next_image);
                    }

                    let mut current_image = pool.alloc().unwrap().init(Image::default());
                    swap(image, &mut current_image);
                    // Replace the image content by the new one, for example
                    // by swapping them, and reset next_pos
                    *next_image = Some(current_image);
                });

                *next_pos = usize::MAX;
            }
        }


        /* 
        cx.shared.next_image.lock(|image, next_image| {
        let next_image: &mut Image = cx.shared.next_image;
        let next_pos: &mut usize = cx.local.next_pos;
        if let Ok(b) = cx.local.usart1_rx.read() {
            // Handle the incoming byte according to the SE203 protocol
            // and update next_image
            // Do not forget that next_image.as_mut() might be handy here!
            if b==0xff || *next_pos==192 {
                *next_pos=0;
            }else{
                let mut image_byte=next_image.as_mut();
                image_byte[*next_pos]= b;
                *next_pos+= 1;
            }
        }


            // If the received image is complete, make it available to
            // the display task.
            if *next_pos == 8 * 8 * 3 {
                cx.shared.image.lock(|image| {
                    // Replace the image content by the new one, for example
                    // by swapping them, and reset next_pos
                    image.clone_from(next_image);
                });
            }

        }
        */
    }
 

    //Removing the rotate_image task
    /* 
    #[task(local = [], shared = [newimage], priority = 1)]
    fn rotate_image(mut cx: rotate_image::Context, at:Instant, color_index:usize) {

    let mut next_l = 0;
    let mut next_index: usize = color_index + 1;
    cx.shared.newimage.lock(|newimage| {
        // Here you can use image, which is a &mut Image,
        // to display the appropriate row
        match color_index {
            0 => *newimage = Image::gradient(RED),
            1 => *newimage = Image::gradient(GREEN),
            2 => *newimage = Image::gradient(BLUE),
            _ => *newimage = Image::gradient(RED),
        }
    if next_index >= 3{
        next_index = 0;
    }
    });
    rotate_image::spawn_after((1.secs())/10, at, next_index).unwrap();
    }

    */

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
