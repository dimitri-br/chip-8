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
    opcode : u16,
    memory : [u8; 4096], 

    registers : [u8; 16], 
    index_register : u16,
    program_counter : u16,


    stack : [u16; 12], 
    stack_ptr : u8,

    audio_timer : u8,
    game_timer : u8, 

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
        let cpu = CPU { memory : [0; 4096], registers : [0; 16], stack : [0; 12], audio_timer: 0, game_timer: 0, index_register: 0, opcode: 0, program_counter: 0x200, stack_ptr: 0 };
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
    pub fn add_to_memory(&mut self, data : u8, location : u16) -> Result<&'static str, &'static str>{
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
    pub fn read_from_memory(&self, location : u16) -> Result<u8, &'static str>{
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
    pub fn set_register(&mut self, register : u16, value : u8) -> Result<(), &'static str>{
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
    pub fn get_register(&self, register : u16) -> Result<u8, &'static str>{
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
    pub fn write_subroutine(&mut self, location : u8, value : u16) -> Result<(), &'static str>{
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
    pub fn read_subroutine(&self, location : u8) -> Result<u16, &'static str>{
        let value : u16 = self.stack[location as usize];
        Ok(value)
    }


    /// # Set Audio timer
    /// 
    /// Set the audio timer variable. This value will control the audio delay.
    /// 
    /// 
    /// 
    /// 
    /// Helpful function
    /// 
    /// # Example
    /// 
    /// ```
    /// 
    /// //set sound timer to 5
    /// cpu.set_audio_timer(0x5).expect("Error setting timer");
    /// ```
    pub fn set_audio_timer(&mut self, value : u8) -> Result<(), &'static str>{
        self.audio_timer = value;
        Ok(())
    }

    /// # Get Audio timer
    /// 
    /// Get the audio timer variable. This value will control the audio delay.
    /// 
    /// 
    /// 
    /// 
    /// Helpful function
    /// 
    /// # Example
    /// 
    /// ```
    /// 
    /// //Get sound timer
    /// cpu.Get_audio_timer().expect("Error Getting timer");
    /// ```
    pub fn get_audio_timer(&self) -> Result<u8, &'static str>{
        let value = self.audio_timer;
        Ok(value)
    }

    /// # Set delay timer
    /// 
    /// Set the game timer. This will control the delay.
    /// 
    /// 
    /// 
    /// 
    /// Helpful function
    /// 
    /// # Example
    /// 
    /// ```
    /// 
    /// //set game timer to 5
    /// cpu.set_game_timer(0x5).expect("Error setting timer");
    /// ```
    pub fn set_game_timer(&mut self, value : u8) -> Result<(), &'static str>{
        self.game_timer = value;
        Ok(())
    }


    /// # Get game time
    /// 
    /// Get the game timer. This will control the delay.
    /// 
    /// 
    /// 
    /// 
    /// Helpful function
    /// 
    /// # Example
    /// 
    /// ```
    /// 
    /// //get game timer
    /// cpu.get_game_timer().expect("Error getting timer");
    /// ```
    pub fn get_game_timer(&self) -> Result<u8, &'static str>{
        let value = self.game_timer;
        Ok(value)
    }

    /// # set the index register
    /// 
    /// Set the index register. This holds values from opcodes.
    /// 
    /// 
    /// 
    /// 
    /// Helpful function
    /// 
    /// # Example
    /// 
    /// ```
    /// 
    /// //set index register to 255
    /// cpu.set_index_register(0xFF).expect("Error setting timer");
    /// ```
    pub fn set_index_register(&mut self, value : u16) -> Result<(), &'static str>{
        self.index_register = value;
        Ok(())
    }

    /// # Read The Index Register
    /// 
    /// Read the index register. This holds values from opcodes.
    /// 
    /// 
    /// 
    /// 
    /// Helpful function
    /// 
    /// # Example
    /// 
    /// ```
    /// 
    /// //read the index register
    /// cpu.read_index_register(0x5).expect("Error setting timer");
    /// ```
    pub fn read_index_register(&self) -> Result<u16, &'static str>{
        let value = self.index_register;
        Ok(value)
    }

    pub fn get_program_counter(&self) -> Result<u16, &'static str>{
        let value = self.program_counter;
        Ok(value)
    }

    /// # WARNING
    /// 
    /// Do not set this manually!
    pub fn set_program_counter(&mut self, value : u16) -> Result<(), &'static str>{
        self.program_counter = value;
        Ok(())
    }
    pub fn set_opcode(&mut self, location : u16) -> Result<(), &'static str>{
        self.opcode = (((self.read_from_memory(location).unwrap() as u64) << 8) + (self.read_from_memory(location).unwrap() as u64)) as u16;
        Ok(())
    }
    pub fn get_opcode(&self) -> Result<u16, &'static str>{
        let value = self.opcode;
        Ok(value)
    }

    pub fn set_stack_pointer(&mut self, value : u8) -> Result<(), &'static str>{
        self.stack_ptr = value;
        Ok(())
    }

    pub fn get_stack_pointer(&self) -> Result<u8, &'static str>{
        let value = self.stack_ptr;
        Ok(value)
    }
}

pub fn run_cpu(cpu : CPU) -> Result<(), &'static str>{
    Ok(())
}