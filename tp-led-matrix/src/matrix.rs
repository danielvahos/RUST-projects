//use stm32l4xx_hal::delay::DelayCM;
//use stm32l4xx_hal::gpio::*;
use crate::{Color, Image};


use stm32l4xx_hal::{rcc::Clocks,gpio::*,prelude::_embedded_hal_blocking_delay_DelayMs};


/* 
use stm32l4xx_hal::gpio::OTYPER;
   |
1  | use stm32l4xx_hal::pac::gpioa::OTYPER;
   |
1  | use stm32l4xx_hal::pac::gpiob::OTYPER;
   |
1  | use stm32l4xx_hal::pac::gpioc::OTYPER;


*/
pub struct Matrix {
    sb: PC5<Output<PushPull>>,
    lat: PC4<Output<PushPull>>,
    rst: PC3<Output<PushPull>>,
    sck: PB1<Output<PushPull>>,
    sda: PA4<Output<PushPull>>,
    c0: PB2<Output<PushPull>>,
    c1: PA15<Output<PushPull>>,
    c2: PA2<Output<PushPull>>,
    c3: PA7<Output<PushPull>>,
    c4: PA6<Output<PushPull>>,
    c5: PA5<Output<PushPull>>,
    c6: PB0<Output<PushPull>>,
    c7: PA3<Output<PushPull>>,
}

impl Matrix {
    /// Create a new matrix from the control registers and the individual
    /// unconfigured pins. SB and LAT will be set high by default, while
    /// other pins will be set low. After 100ms, RST will be set high, and
    /// the bank 0 will be initialized by calling `init_bank0()` on the
    /// newly constructed structure.
    /// The pins will be set to very high speed mode.
    #[allow(clippy::too_many_arguments)]   // Necessary to avoid a clippy warning
    pub fn new(
        pa2: PA2<Analog>,
        pa3: PA3<Analog>,
        pa4: PA4<Analog>,
        pa5: PA5<Analog>,
        pa6: PA6<Analog>,
        pa7: PA7<Analog>,
        pa15: PA15<Alternate<PushPull, 0>>,
        pb0: PB0<Analog>,
        pb1: PB1<Analog>,
        pb2: PB2<Analog>,
        pc3: PC3<Analog>,
        pc4: PC4<Analog>,
        pc5: PC5<Analog>,
        gpioa_moder: &mut MODER<'A'>,
        gpioa_otyper: &mut OTYPER<'A'>,
        gpiob_moder: &mut MODER<'B'>,
        gpiob_otyper: &mut OTYPER<'B'>,
        gpioc_moder: &mut MODER<'C'>,
        gpioc_otyper: &mut OTYPER<'C'>,
        clocks: Clocks,
    ) -> Self {
        // Use .into_push_pull_output_in_state(…) to set an initial state on pins

        let mut matrix:Matrix=Matrix{
            sb:pc5.into_push_pull_output_in_state(gpioc_moder,gpioc_otyper,PinState::High).set_speed(Speed::VeryHigh),
            lat:pc4.into_push_pull_output_in_state(gpioc_moder,gpioc_otyper,PinState::High).set_speed(Speed::VeryHigh),
            rst:pc3.into_push_pull_output_in_state(gpioc_moder,gpioc_otyper,PinState::Low).set_speed(Speed::VeryHigh),
            sck:pb1.into_push_pull_output_in_state(gpiob_moder,gpiob_otyper,PinState::Low).set_speed(Speed::VeryHigh),
            sda:pa4.into_push_pull_output_in_state(gpioa_moder,gpioa_otyper,PinState::Low).set_speed(Speed::VeryHigh),
            c0:pb2.into_push_pull_output_in_state(gpiob_moder,gpiob_otyper,PinState::Low).set_speed(Speed::VeryHigh),
            c1:pa15.into_push_pull_output_in_state(gpioa_moder,gpioa_otyper,PinState::Low).set_speed(Speed::VeryHigh),
            c2:pa2.into_push_pull_output_in_state(gpioa_moder,gpioa_otyper,PinState::Low).set_speed(Speed::VeryHigh),
            c3:pa7.into_push_pull_output_in_state(gpioa_moder,gpioa_otyper,PinState::Low).set_speed(Speed::VeryHigh),
            c4:pa6.into_push_pull_output_in_state(gpioa_moder,gpioa_otyper,PinState::Low).set_speed(Speed::VeryHigh),
            c5:pa5.into_push_pull_output_in_state(gpioa_moder,gpioa_otyper,PinState::Low).set_speed(Speed::VeryHigh),
            c6:pb0.into_push_pull_output_in_state(gpiob_moder,gpiob_otyper,PinState::Low).set_speed(Speed::VeryHigh),
            c7:pa3.into_push_pull_output_in_state(gpioa_moder,gpioa_otyper,PinState::Low).set_speed(Speed::VeryHigh),
            
        };
        let mut delay=stm32l4xx_hal::delay::DelayCM::new(clocks);
        delay.delay_ms(100u8);
        matrix.rst.set_high();
        matrix.init_bank0();
        matrix

        /*
        gpioc.pc5.into_push_pull_output.sethigh(); //SB
        gpioc.pc4.into_push_pull_output.sethigh(); //LAT
        gpioc.pc3.into_push_pull_output.setlow(); //RST
        gpiob.pb1.into_push_pull_output.setlow(); //SCK
        gpioa.pa4.into_push_pull_output.setlow(); //SDA
        gpiob.pb2.into_push_pull_output.setlow(); //C0
        gpioa.pa15.into_push_pull_output.setlow(); //C1
        gpioa.pa2.into_push_pull_output.setlow(); //C2
        gpioa.pa7.into_push_pull_output.setlow(); //C3
        gpioa.pa6.into_push_pull_output.setlow(); //C4
        gpioa.pa5.into_push_pull_output.setlow(); //C5
        gpiob.pb0.into_push_pull_output.setlow(); //C6
        gpioa.pa3.into_push_pull_output.setlow(); //C7

        */
    }

    /// Make a brief high pulse of the SCK pin
    fn pulse_sck(&mut self){
        self.sck.set_high();
        self.sck.set_low();
    }

    /// Make a brief low pulse of the LAT pin
    fn pulse_lat(&mut self) {
        self.lat.set_low();
        self.lat.set_high();
    }

    /// Set the given row output in the chosen state
    fn row(&mut self, row: usize, state: PinState) {
        match row{
            0 =>self.c0.set_state(state), //C0
            1 =>self.c1.set_state(state), //C0
            2 =>self.c2.set_state(state), //C0
            3 =>self.c3.set_state(state), //C0
            4 =>self.c4.set_state(state), //C0
            5 =>self.c5.set_state(state), //C0
            6 =>self.c6.set_state(state), //C0
            7 =>self.c7.set_state(state), //C0
            _ => unreachable!(), //C0
        }
    }

    /// Send a byte on SDA starting with the MSB(M significant byte) and pulse SCK high after each bit
    fn send_byte(&mut self, pixel: u8) {
        /* 
        for i in 0..8{
            if (pixel>>(7-i) &1) == 0{
                gpioa.pa4.into_push_pull_output.setlow()
            }
            else{
                gpioa.pa4.into_push_pull_output.sethigh()
            }
            pulse_sck()
        }
        */
        for i in (0..8).rev(){
            self.sda.set_state((pixel &(1<<i) != 0).into());
            self.pulse_sck();
        }
        
    }

    /* 
    pub fn deactivate_rows(&mut self){

        gpiob.pb2.into_push_pull_output.setlow(); //C0
        gpioa.pa15.into_push_pull_output.setlow(); //C1
        gpioa.pa2.into_push_pull_output.setlow(); //C2
        gpioa.pa7.into_push_pull_output.setlow(); //C3
        gpioa.pa6.into_push_pull_output.setlow(); //C4
        gpioa.pa5.into_push_pull_output.setlow(); //C5
        gpiob.pb0.into_push_pull_output.setlow(); //C6
        gpioa.pa3.into_push_pull_output.setlow(); //C7
    }

    pub fn activate_row(&mut self, row: usize){
        match row{
            0=>gpiob.pb2.into_push_pull_output.sethigh(),//C0
            1=>gpioa.pa15.into_push_pull_output.sethigh(),
            2=>gpioa.pa2.into_push_pull_output.sethigh(),
            3=>gpioa.pa7.into_push_pull_output.sethigh(),
            4=>gpioa.pa6.into_push_pull_output.sethigh(),
            5=>gpioa.pa5.into_push_pull_output.sethigh(),
            6=>gpiob.pb0.into_push_pull_output.sethigh(),
            7=>gpioa.pa3.into_push_pull_output.sethigh(),
        }
    }

    */

    /// Send a full row of bytes in BGR order and pulse LAT low. Gamma correction
    /// must be applied to every pixel before sending them. The previous row must
    /// be deactivated and the new one activated.
    pub fn send_row(&mut self, row: usize, pixels: &[Color]) {
        for i in 0..8{
            //With the order BGR
            self.send_byte(pixels[i].gamma_correct().b); //Gamma correct possibly to delete
            self.send_byte(pixels[i].gamma_correct().g);
            self.send_byte(pixels[i].gamma_correct().r);
        }
        self.row((row+7)%8, PinState::Low);
        self.pulse_lat();
        self.row(row, PinState::High);
        /*deactivate_rows();
        DelayCM(10);

        pulse_LAT();
        activate_row(row);*/
    }

    /// Initialize bank0 by temporarily setting SB to low and sending 144 one bits,
    /// pulsing SCK high after each bit and pulsing LAT low at the end. SB is then
    /// restored to high.
    fn init_bank0(&mut self) {
        self.sb.set_low();
        self.sda.set_high();
        //gpioc.pc5.into_push_pull_output.setlow(); //SB
        //144/8=18, each byte sends 8 bits to bank0, being 144 in total

        /*
        for i in 0..144{
            self.pulse_sck();
        }
        */

        for i in 0..18{
            self.send_byte(255)
        }
        self.pulse_lat();
        self.sb.set_high();
    }


    //Build an image made of grandient blue and display it in loop on the matrix


    /// Display a full image, row by row, as fast as possible.
    pub fn display_image(&mut self, image: &Image) {
        for i in 0..8{
            self.send_row(i, image.row(i));
        }
        // Do not forget that image.row(n) gives access to the content of row n,
        // and that self.send_row() uses the same format.
    }
}
