use std::io::{self, Write};

const MEMORY_SIZE: usize = 4096;
const NREGS: usize = 16;

const IP: usize = 0;

pub struct Machine {
    // Implement me!
    t_memory: [u8; MEMORY_SIZE],
    t_register: [u32; NREGS],
}

#[derive(Debug)]
pub enum MachineError {
    // Add some entries to represent errors!
    IncorrectValue,
    IncorrectRegRange,
    IncorrectMemRange,
    RegNotAllowed,
    ErrorOutput,
    InstructionError,
}

impl Machine {
    /// Create a new machine in its reset state. The `memory` parameter will
    /// be copied at the beginning of the machine memory.
    ///
    /// # Panics
    /// This function panics when `memory` is larger than the machine memory.
    pub fn new(memory: &[u8]) -> Self {
        let mut machine = Self{
            t_memory:[0; MEMORY_SIZE],//new memory in reset state
            t_register:[0;NREGS],
        };
        machine.t_memory[..memory.len()].copy_from_slice(memory); //copy to the machine the whole memory as argument
        // It has to be the length of the new memory, for putting into the current machine memory
        return machine; //return the machine with the new memory as argument

        //unimplemented!()  // Implement me!
    }

    /// Run until the program terminates or until an error happens.
    /// If output instructions are run, they print on `fd`.
    pub fn run_on<T: Write>(&mut self, fd: &mut T) -> Result<(), MachineError> {
        while !self.step_on(fd)? {}
        Ok(())
    }

    /// Run until the program terminates or until an error happens.
    /// If output instructions are run, they print on standard output.
    pub fn run(&mut self) -> Result<(), MachineError> {
        self.run_on(&mut io::stdout().lock())
    }

    /// Execute the next instruction by doing the following steps:
    ///   - decode the instruction located at IP (register 0)
    ///   - increment the IP by the size of the instruction
    ///   - execute the decoded instruction
    ///
    /// If output instructions are run, they print on `fd`.
    /// If an error happens at either of those steps, an error is
    /// returned.
    ///
    /// In case of success, `true` is returned if the program is
    /// terminated (upon encountering an exit instruction), or
    /// `false` if the execution must continue.
    pub fn step_on<T: Write>(&mut self, fd: &mut T) -> Result<bool, MachineError> {

        let address: usize = self.t_register[0] as usize; //it's register 0 because it's the number of instruction
        let opcode = self.t_memory[address];
        match opcode {
            1 => {
                self.t_register[0] = self.t_register[0] + 4;

                let regA : u8 = self.get_mem(address + 1)?;
                let regB : u8 = self.get_mem(address + 2)?;
                let regC : u8 = self.get_mem(address + 3)?;

                self.moveif(regA, regB, regC);
                Ok(false)
            },
            2 => {
                self.t_register[0] = self.t_register[0] + 3;

                let regA : u8 = self.get_mem(address + 1)?;
                let regB : u8 = self.get_mem(address + 2)?;

                self.store(regA, regB);
                Ok(false)
            },
            3 => {
                self.t_register[0] = self.t_register[0] + 3;

                let regA : u8 = self.get_mem(address + 1)?;
                let regB : u8 = self.get_mem(address + 2)?;

                self.load(regA, regB);
                Ok(false)
            },
            4 => {
                self.t_register[0]= self.t_register[0] + 4;

                let regA : u8 = self.get_mem(address + 1)?;
                let L : u8 = self.get_mem(address + 2)?;
                let H : u8 = self.get_mem(address + 3)?;

                self.loadimm(regA, L, H);
                Ok(false)
            },
            5 =>{
                self.t_register[0] = self.t_register[0]+ 4;

                let regA : u8 = self.get_mem(address + 1)?;
                let regB : u8 = self.get_mem(address + 2)?;
                let regC : u8 = self.get_mem(address + 3)?;

                self.sub(regA, regB, regC);
                Ok(false)
            },
            6 =>{
                self.t_register[0] = self.t_register[0] + 2;

                let regA : u8 = self.get_mem(address + 1)?;

                self.out(regA,fd)?;
                Ok(false)
            },
            7 => {
                self.t_register[0] = self.t_register[0] + 1;
                Ok(true)
                },
            8 => {
                self.t_register[0] = self.t_register[0] + 2;

                let regA : u8 = self.get_mem(address + 1)?;

                self.outnumber(regA,fd)?;
                Ok(false)
            },

            _ => Err(MachineError::InstructionError)
        }
    }

    //          INSTRUCTIONS

    // 1) MOVE IF INSTRUCTION
    pub fn moveif(&mut self, regA: u8, regB: u8, regC: u8){
        if self.t_register[regC as usize] != 0 {
            self.t_register[regA as usize] = self.t_register[regB as usize];
        }
        else{}
    }

    // 2) STORE
    pub fn store(&mut self, regA: u8, regB: u8){
        let value = self.t_register[regB as usize];

        self.t_memory[self.t_register[regA as usize] as usize] = (value >> 24) as u8;
        self.t_memory[self.t_register[(regA  + 1) as usize] as usize] = ((value >> 16) & 0xFF) as u8;
        self.t_memory[self.t_register[(regA + 2) as usize] as usize] = ((value >> 8) & 0xFF) as u8;
        self.t_memory[self.t_register[(regA + 3) as usize] as usize] = (value & 0xFF) as u8;
    }

    // 3) LOAD
    pub fn load(&mut self, regA: u8, regB: u8){
        self.t_memory[self.t_register[regA as usize] as usize]= self.t_memory[regB as usize];
    }

    // 4) LOADIMM
    pub fn loadimm(&mut self, regA: u8, L: u8, H: u8){
        self.t_register[regA as usize] = i32::from(
            (((H as u16) << 8) | L as u16) as i16) as u32;
    }

    //5) SUB
    pub fn sub(&mut self, regA: u8, regB: u8, regC: u8){
        self.t_register[regA  as usize] = self.t_register[regB as usize] - self.t_register[regC as usize];
    }

    //6) OUT
    pub fn out<T: Write>(&mut self, regA: u8,fd: &mut T) -> Result<(), MachineError>{
        let val = self.t_register[regA as usize] & 0xFF;
        let unicode = val as u8 as char;

        let result = fd.write(format!("{}", unicode).as_bytes());

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(MachineError::ErrorOutput)
        }
    }

    //7) EXIT --> it's not necessary a function, it works only with Ok(True)

    //8) OUT NUMBER
    pub fn outnumber<T: Write>(&mut self, regA: u8, fd: &mut T) -> Result<(), MachineError>{
        let val = self.t_register[regA as usize] as i32;

        let result = fd.write(format!("{}", val).as_bytes());

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(MachineError::ErrorOutput)
        }
    }

//-----------------------------------------------------------------------------

    /// Similar to [step_on](Machine::step_on).
    /// If output instructions are run, they print on standard output.
    pub fn step(&mut self) -> Result<bool, MachineError> {
        self.step_on(&mut io::stdout().lock())
    }

    /// Reference onto the machine current set of registers.
    pub fn regs(&self) -> &[u32] {
        //unimplemented!()  // Implement me!
        return &self.t_register; //return of registers current state
    }

    /// Sets a register to the given value.
    pub fn set_reg(&mut self, reg: usize, value: u32) -> Result<(), MachineError> {

        //C'EST POSSIBLE AVEC R0
        if reg == 0{
            return Err(MachineError:: RegNotAllowed);
        }
        else if NREGS < reg{
            return Err(MachineError::IncorrectRegRange);
        }
        else {
            self.t_register[reg]=value;
                return Ok(println!("The value was set to the register {}", reg))
        }
        //unimplemented!()  // Implement me!
    }

    /// Reference onto the machine current memory.
    pub fn memory(&self) -> &[u8] {
        //unimplemented!()  // Implement me!
        return &self.t_memory; //return the memory current state
    }
    // fair une fonction pour lire memorie (&self.get_mem ory) va returner RESULT DE  u8 OU machine error
    pub fn get_mem (&self, addr: usize)-> Result<u8, MachineError> {
        if addr > MEMORY_SIZE{
            return Err(MachineError::IncorrectMemRange);
        }
        else{
            Ok(self.t_memory[addr])
        }
    }
}
