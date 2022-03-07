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
    RegNotAllowed,

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

        let inst: u32 = self.t_register[0]; //it's register 0 because it's the number of instruction
        let mut inst_next: u32=
        match inst{
            1 => self.t_register[0] + 4,
            2 => self.t_register[0] + 3,
            3 => self.t_register[0] + 3,
            4 => self.t_register[0] + 4,
            5 => self.t_register[0] + 4,
            6 => self.t_register[0] + 2,
            7 => self.t_register[0] + 1,
            8 => self.t_register[0] + 2,
        }
        
        //unimplemented!()  // Implement me!
    }

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
}
