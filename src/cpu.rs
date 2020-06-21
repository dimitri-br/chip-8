/// # The CPU struct
/// 
/// This will contain all our data for our CPU, such as the memory and registers.
/// 
/// It make it easier to access and change the data, with helpful functions to generate a new CPU instance.
/// 
/// 
/// # Important Information About Chip-8
/// Memory size is 4kb
/// 
/// 16 registers from V0 - VF, each register can contain any hexadecimal number from 0x00 to 0xFF
/// 
/// Space for subroutine calls. Must have >=12
/// 
/// Timer will control delay of frames and sound.
/// 
/// 
/// # Example
/// 
/// ```
/// //create a cpu
/// let mut cpu = CPU::new().expect("Error creating CPU instance"); 
/// 
/// //add to cpu memory
/// cpu.add_to_memory(32, 0).expect("Error writing to memory");
/// 
/// //read from memory
/// let mem_value = cpu.read_from_memory(0).expect("Error reading from memory");
///
/// //write register
/// cpu.write_register(0xA, mem_value).expect("Error writing to register");
/// 
/// //read register
/// let reg_value = cpu.read_register(0xA).expect("Error reading from register");
///
/// //write to stack
/// cpu.write_subroutine(0x00, reg_value).expect("Error writing to stack");
/// 
/// //read from stack
/// let stack_value = cpu.read_subroutine(0x00).expect("Error reading from stack");
///
/// //set timer
/// cpu.set_timer(1, stack_value).expect("Error setting timer");
/// 
/// 
/// ```
/// 


pub struct CPU{
    memory : [u8; 4096], 
    registers : [u8; 16], 
    stack : [u8; 12], 
    timers : [u8; 2], 
}


impl CPU{
    /// # Create a new CPU
    /// 
    /// This function will generate an empty CPU in a result.
    /// 
    /// # Example
    /// 
    /// ```
    /// //create a cpu
    /// let mut cpu = CPU::new().expect("Error creating CPU instance");     
    /// ```
    pub fn new() -> Result<Self, &'static str>{
        let cpu = CPU { timers : [60, 60], memory : [0; 4096], registers : [0; 16], stack : [0; 12] };
        Ok(cpu)
    }

    /// # Add to memory
    /// 
    /// This function will add a `u8` data type to a memory location, also stored as a `u8`
    /// 
    /// # Example
    /// 
    /// ```
    /// //write value '32' to memory location '0'
    /// cpu.add_to_memory(32, 0).expect("Error writing to memory");
    /// ```
    pub fn add_to_memory(&mut self, data : u8, location : u8) -> Result<&'static str, &'static str>{
        self.memory[location as usize] = data;
        Ok("Successfully wrote to memory")
    }

    /// # Read from memory
    /// 
    /// This function will read a ```u8``` value from a memory location, stored as a ```u8```
    /// 
    /// # Example
    /// 
    /// ```
    /// //read value from memory location '0'
    /// let value = cpu.read_from_memory(0).expect("Error reading from memory");
    /// ```
    pub fn read_from_memory(&self, location : u8) -> Result<u8, &'static str>{
        let read_value : u8 = self.memory[location as usize];
        Ok(read_value)
    }


    /// # Write to register
    /// 
    /// Write to register location. Replace current value with new value
    /// 
    /// Helpful function
    /// 
    /// # Example
    /// 
    /// ```
    /// 
    /// //write 5 to register VA to memory
    /// cpu.write_register(10, 5).expect("Error writing to register");
    /// ```
    pub fn write_register(&mut self, register : u8, value : u8) -> Result<(), &'static str>{
        self.registers[register as usize] = value;
        Ok(())
    }

    /// # Read from register
    /// 
    /// Read from register. Get a u8 value from an address
    /// 
    /// Helpful function
    /// 
    /// # Example
    /// 
    /// ```
    /// 
    /// //read from location VA
    /// let value = cpu.read_register(0xA).expect("Error writing to register");
    /// ```
    pub fn read_register(&self, register : u8) -> Result<u8, &'static str>{
        let value : u8 = self.registers[register as usize];
        Ok(value)
    }

    /// # Write a subroutine
    /// 
    /// Write a subroutine to the stack. This is an instruction to run.
    /// This will overwrite any instrution currently on the stack
    /// Helpful function
    /// 
    /// # Example
    /// 
    /// ```
    /// 
    /// //write new subroutine to stack
    /// cpu.write_subroutine(0x00, 0xA).expect("Error writing to stack");
    /// ```
    pub fn write_subroutine(&mut self, location : u8, value : u8) -> Result<(), &'static str>{
        self.stack[location as usize] = value;
        Ok(())
    }

    /// # Read a subroutine
    /// 
    /// Read a subroutine from the stack. This is an instruction to run.
    /// 
    /// Helpful function
    /// 
    /// # Example
    /// 
    /// ```
    /// 
    /// //read current subroutine from stack
    /// cpu.read_subroutine(0x00, 0xA).expect("Error reading to stack");
    /// ```
    pub fn read_subroutine(&self, location : u8) -> Result<u8, &'static str>{
        let value : u8= self.stack[location as usize];
        Ok(value)
    }


    /// # Set a timer
    /// 
    /// Set a timer variable. This value can be either:
    /// 
    /// 0 - Game Timer. This controls the flow of time (typically 60hz)
    /// 
    /// 1 - Sound Timer. This controls the delay between sound.
    /// 
    /// Any other number will return an error.
    /// 
    /// Helpful function
    /// 
    /// # Example
    /// 
    /// ```
    /// 
    /// //set sound timer to 5
    /// cpu.set_timer(1, 0x5).expect("Error setting timer");
    /// ```
    pub fn set_timer(&mut self, timer : u8, value : u8) -> Result<(), &'static str>{
        match timer{
            0 => self.timers[0] = value,
            1 => self.timers[1] = value,
            _ => return Err("Error!      Only 2 timers availiable!      0 - Game Delay        1 - Sound Delay")
        }

        Ok(())
    }
    
}