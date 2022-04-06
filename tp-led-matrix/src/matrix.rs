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
        gpioc.pc5.into_push_pull_output_in_state.sethigh(); //SB
        gpioc.pc4.into_push_pull_output_in_state.sethigh(); //LAT
        gpioc.pc3.into_push_pull_output_in_state.setlow(); //RST
        gpiob.pb1.into_push_pull_output_in_state.setlow(); //SCK
        gpioa.pa4.into_push_pull_output_in_state.setlow(); //SDA
        gpiob.pb2.into_push_pull_output_in_state.setlow(); //C0
        gpioa.pa15.into_push_pull_output_in_state.setlow(); //C1
        gpioa.pa2.into_push_pull_output_in_state.setlow(); //C2
        gpioa.pa7.into_push_pull_output_in_state.setlow(); //C3
        gpioa.pa6.into_push_pull_output_in_state.setlow(); //C4
        gpioa.pa5.into_push_pull_output_in_state.setlow(); //C5
        gpiob.pb0.into_push_pull_output_in_state.setlow(); //C6
        gpioa.pa3.into_push_pull_output_in_state.setlow(); //C7
    }

    /// Make a brief high pulse of the SCK pin
    fn pulse_sck(&mut self) {
        gpiob.pb1.into_push_pull_output_in_state.sethigh(); //SCK
    }

    /// Make a brief low pulse of the LAT pin
    fn pulse_lat(&mut self) {
        gpioc.pc4.into_push_pull_output_in_state.setlow(); //LAT
    }

    /// Set the given row output in the chosen state
    fn row(&mut self, row: usize, state: PinState) {
        match row{
            0 =>gpiob.pb2.into_push_pull_output_in_state.states(), //C0
            1 =>gpioa.pa15.into_push_pull_output_in_state.state(), //C1
            2 =>gpioa.pa2.into_push_pull_output_in_state.state(), //C2
            3 =>gpioa.pa7.into_push_pull_output_in_state.state(), //C3
            4 =>gpioa.pa6.into_push_pull_output_in_state.state(), //C4
            5 =>gpioa.pa5.into_push_pull_output_in_state.state(), //C5
            6 =>gpiob.pb0.into_push_pull_output_in_state.state(), //C6
            7 => gpioa.pa3.into_push_pull_output_in_state.state(), //C7
        }
    }

    /// Send a byte on SDA starting with the MSB(M significant byte) and pulse SCK high after each bit
    fn send_byte(&mut self, pixel: u8) {
        for i in 0..8{
            if (pixel>>(7-i) &1) == 0{
                gpioa.pa4.into_push_pull_output_in_state.setlow()
            }
            else{
                gpioa.pa4.into_push_pull_output_in_state.sethigh()
            }
            pulse_sck()
        }
    }

    /// Send a full row of bytes in BGR order and pulse LAT low. Gamma correction
    /// must be applied to every pixel before sending them. The previous row must
    /// be deactivated and the new one activated.
    pub fn send_row(&mut self, row: usize, pixels: &[Color]) {
        todo!()
    }

    /// Initialize bank0 by temporarily setting SB to low and sending 144 one bits,
    /// pulsing SCK high after each bit and pulsing LAT low at the end. SB is then
    /// restored to high.
    fn init_bank0(&mut self) {
        todo!()
    }

    /// Display a full image, row by row, as fast as possible.
    pub fn display_image(&mut self, image: &Image) {
        // Do not forget that image.row(n) gives access to the content of row n,
        // and that self.send_row() uses the same format.
        todo!()
    }
}
