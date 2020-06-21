mod read;
use read::Reader;

const chip8_fontset : [u8; 5 * 16] =
[
    0xF0, 0x90, 0x90, 0x90, 0xF0, //0
    0x20, 0x60, 0x20, 0x20, 0x70, //1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, //2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, //3
    0x90, 0x90, 0xF0, 0x10, 0x10, //4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, //5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, //6
    0xF0, 0x10, 0x20, 0x40, 0x40, //7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, //8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, //9
    0xF0, 0x90, 0xF0, 0x90, 0x90, //A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, //B
    0xF0, 0x80, 0x80, 0x80, 0xF0, //C
    0xE0, 0x90, 0x90, 0x90, 0xE0, //D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, //E
    0xF0, 0x80, 0xF0, 0x80, 0x80  //F
];

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
/// Timers will control delay of frames and sound.
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

    gfx : [u8; 64 * 32],

    keys : [u8; 16]

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
        let mut cpu = CPU { memory : [0; 4096], registers : [0; 16], stack : [0; 12], audio_timer: 0, game_timer: 0, index_register: 0, opcode: 0, program_counter: 0x200, stack_ptr: 0, gfx: [0; 64 * 32], keys : [0; 16] };
        
        
        
        //load charset into memory
        for i in 0..80{
            cpu.set_memory(chip8_fontset[i], i as u16).unwrap();
        }
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
    pub fn set_memory(&mut self, data : u8, location : u16) -> Result<&'static str, &'static str>{
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
    pub fn get_memory(&self, location : u16) -> Result<u8, &'static str>{
        let read_value = self.memory[location as usize];
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
        let value = self.registers[register as usize];
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
    pub fn get_index_register(&self) -> Result<u16, &'static str>{
        let value = self.index_register;
        Ok(value)
    }


    /// # Program counter
    /// 
    /// This will contain the current instruction being run by the CPU.
    /// 
    /// It will get the instruction from memory.
    /// 
    /// It will use its stored number as a memory address
    /// 
    /// # Example
    /// 
    /// ```
    /// //get program counter
    /// cpu.get_program_counter().expect("Error getting program counter");
    /// 
    /// ```
    pub fn get_program_counter(&self) -> Result<u16, &'static str>{
        let value = self.program_counter;
        Ok(value)
    }

    /// # Program counter
    /// 
    /// This will contain the current instruction being run by the CPU.
    /// 
    /// It will get the instruction from memory.
    /// 
    /// It will use its stored number as a memory address
    /// 
    /// # Example
    /// 
    /// ```
    /// //set program counter to 255
    /// cpu.set_program_counter(0xFF).expect("Error setting program counter");
    /// 
    /// ```
    pub fn set_program_counter(&mut self, value : u16) -> Result<(), &'static str>{
        self.program_counter = value;
        Ok(())
    }

    /// # Set Opcode
    /// 
    /// This will set the opcode based on the current program counter
    /// 
    /// # Example
    /// 
    /// ```
    /// 
    /// //set opcode
    /// cpu.set_opcode(cpu.get_program_counter().unwrap()).expect("Error setting opcode");
    /// 
    /// ```
    pub fn set_opcode(&mut self, location : u16) -> Result<(), &'static str>{
        self.opcode = (((self.get_memory(location).unwrap() as u64) << 8) | (self.get_memory(location + 1).unwrap() as u64)) as u16;
        Ok(())
    }


    /// # Get Opcode
    /// 
    /// This will get the opcode
    /// 
    /// # Example
    /// 
    /// ```
    /// 
    /// //get opcode
    /// cpu.get_opcode().expect("Error getting opcode");
    /// 
    /// ```
    pub fn get_opcode(&self) -> Result<u16, &'static str>{
        let value = self.opcode;
        Ok(value)
    }


    /// # Set stack pointer
    /// 
    /// The stack pointer controls where the program is in the stack
    /// 
    /// The stack is used to store the current PC when running a subroutine
    /// 
    /// # Example
    /// 
    /// ```
    /// //add pc to stack and set new stack pointer
    /// let sp = cpu.get_stack_pointer().unwrap(); //get current stack pointer
    /// cpu.write_subroutine(sp, cpu.get_program_counter().unwrap()).unwrap(); //save our current address to the stack     
    /// cpu.set_stack_pointer(sp + 1).unwrap(); //set new stack pointer + 1 so we don't overwrite
    /// cpu.set_program_counter(opcode & 0x0FFF).unwrap(); //set PC to new subroutine location
    /// ```
    pub fn set_stack_pointer(&mut self, value : u8) -> Result<(), &'static str>{
        self.stack_ptr = value;
        Ok(())
    }

    /// # Get stack pointer
    /// 
    /// This will get the current stack pointer position
    /// 
    /// # Example
    /// 
    /// ```
    /// let sp = cpu.get_stack_pointer().unwrap(); //get current stack pointer
    /// ```
    pub fn get_stack_pointer(&self) -> Result<u8, &'static str>{
        let value = self.stack_ptr;
        Ok(value)

    }


    /// # Get GFX
    /// 
    /// This will get a pixel value at location `u16`
    /// 
    /// Useful for reading the screen data
    /// 
    /// # Example
    /// 
    /// ```
    /// //get pixel at 20
    /// cpu.get_gfx(20).expect("Error getting gfx");
    /// ```
    pub fn get_gfx(&self, location : u16) -> Result<u8, &'static str>{
        let value = self.gfx[location as usize];
        Ok(value)
    }

    /// # Set GFX
    /// 
    /// This will set a pixel value at location `u16`
    /// 
    /// Useful for drawing to screen
    /// 
    /// # Example
    /// 
    /// ```
    /// //set pixel at 20 to 1 (on)
    /// cpu.get_gfx(20, 1).expect("Error setting gfx");
    /// ```
    pub fn set_gfx(&mut self, location : u16, value :u8) -> Result<(), &'static str>{
        self.gfx[location as usize] = value;
        Ok(())
    }

    /// # Clear GFX
    /// 
    /// This function will clear the screen
    /// 
    /// # Example
    /// ```
    /// //clear screen
    /// cpu.clr_gfx().unwrap();
    /// ```
    pub fn clr_gfx(&mut self) -> Result<(), &'static str>{
        self.gfx = [0; 64 *32];
        Ok(())
    }


    /// # Set Key
    /// 
    /// this function will set a key's value
    /// 
    /// # Example
    /// ```
    /// //set key at locaton 0 to on
    /// cpu.set_key(0, 1).unwrap();
    /// ```
    pub fn set_key(&mut self, location : u8, value : u8) -> Result<(), &'static str>{
        self.keys[location as usize] = value;
        Ok(())
    }


    /// # Get Key
    /// 
    /// this function will read a key's value at location
    /// 
    /// # Example
    /// 
    /// ``` 
    /// //read key 0
    /// cpu.get_key(0).unwrap()
    /// ```
    pub fn get_key(&self, location : u8) -> Result<u8, &'static str>{
        let value = self.keys[location as usize];
        Ok(value)
    }

}



/// # Load ROM
/// 
/// this function will load a ROM from `/programs`
/// 
/// It will automatically write the ROM to memory
/// 
/// It requires a mutable instance of a CPU, and will return CPU
/// 
/// # Example
/// 
/// ```
/// //read a rom
/// let mut cpu = CPU::new().unwrap();
/// cpu = load_rom(cpu).unwrap();
/// ```
pub fn load_rom(mut cpu : CPU) -> Result<CPU, &'static str>{
    let mut reader = Reader::new("programs/life.ch8").unwrap();
    let mut cpu = CPU::new().expect("Error creating CPU instance");
    reader.open().unwrap();
    let mut current_mem = 0x200;
    cpu.set_program_counter(current_mem);
    for line in reader.content.iter(){
        cpu.set_memory(*line, current_mem).unwrap();
        current_mem += 1;
    }
    cpu.set_opcode(cpu.get_program_counter().unwrap()).unwrap();
    Ok(cpu)

}



/// # Emulate Cycle
/// 
/// This function will run through a CPU Fetch, decode execute cycle
/// 
/// It will automatically run a program when called
/// 
/// Set key values and read GFX values in an external program
/// 
/// The cycle will automatically read and write the values
/// 
/// It requires a mutable instance of a CPU, and will return a CPU
/// 
/// # Example
/// 
/// ```
/// //create a cpu, load a program and run
/// let mut cpu = CPU::new().unwrap();
/// cpu = load_rom(cpu);
/// loop {
///     cpu = execute_cycle(cpu);
/// }
/// ```
pub fn emulate_cycle(mut cpu : CPU) -> Result<CPU, &'static str>{
    cpu.set_opcode(cpu.get_program_counter().unwrap()).unwrap();
    let opcode = cpu.get_opcode().unwrap();
    println!("{:#x?}",opcode);
    
    match opcode & 0xF000{
        0x0000 =>{
            
            match opcode & 0x000F{
                0x0000 => {
                    cpu.clr_gfx().unwrap(); //0x00E0
                },

                0x000E => {
                    cpu.set_stack_pointer(cpu.get_stack_pointer().unwrap() - 1).unwrap();
                    cpu.set_program_counter(cpu.read_subroutine(cpu.get_stack_pointer().unwrap()).unwrap()).unwrap();
                    cpu.set_program_counter(cpu.get_program_counter().unwrap() + 2).unwrap();
                },
                _ => println!("Opcode {:#x?} is not implemented", opcode)

            }
        },
        0x1000..=0x1FFF => { //jumps to 1NNN
            cpu.set_program_counter(opcode & 0x0FFF).unwrap();
        },
        0x2000..=0x2FFF =>{
            //this opcode runs a subroutine
            let sp = cpu.get_stack_pointer().unwrap(); //get current stack pointer
            cpu.write_subroutine(sp, cpu.get_program_counter().unwrap()).unwrap(); //save our current address to the stack
            
          
           
            cpu.set_stack_pointer(sp + 1).unwrap(); //set new stack pointer + 1 so we don't overwrite
            
            
            cpu.set_program_counter(opcode & 0x0FFF).unwrap(); //set PC to new subroutine location
            
        },
        0x3000..=0x3FFF => {
            if cpu.get_register((opcode & 0x0F00) >> 8).unwrap() as u16 == (opcode & 0x00FF){   
                cpu.set_program_counter(cpu.get_program_counter().unwrap() + 4).unwrap();
            }else{
                cpu.set_program_counter(cpu.get_program_counter().unwrap() + 2).unwrap();
            }
        },

        0x4000..=0x4FFF => {
            if cpu.get_register((opcode & 0x0F00) >> 8).unwrap() as u16 != (opcode & 0x00FF){   
                cpu.set_program_counter(cpu.get_program_counter().unwrap() + 4).unwrap();
            }else{
                cpu.set_program_counter(cpu.get_program_counter().unwrap() + 2).unwrap();
            }
        },
        
        0x5000..=0x5FFF => {
            if cpu.get_register((opcode & 0x0F00) >> 8).unwrap() == cpu.get_register((opcode & 0x00F0) >> 4).unwrap(){   
                cpu.set_program_counter(cpu.get_program_counter().unwrap() + 4).unwrap();
            }else{
                cpu.set_program_counter(cpu.get_program_counter().unwrap() + 2).unwrap();
            }
        },

        0x6000..=0x6FFF => {
            cpu.set_register((opcode & 0x0F00) >> 8, (opcode & 0x00FF) as u8).unwrap();
            cpu.set_program_counter(cpu.get_program_counter().unwrap() + 2).unwrap();
        },

        0x7000..=0x7FFF => {
            cpu.set_register((opcode & 0x0F00) >> 8, cpu.get_register((opcode & 0x0F00) >> 8).unwrap() + (opcode & 0x00FF) as u8).unwrap();
            cpu.set_program_counter(cpu.get_program_counter().unwrap() + 2).unwrap();
        },

        0x8000..=0x8FFF => {
            match opcode & 0x000F{
                0x0000 =>{ // 0x8X Y0: Sets VX to the value of VY
                    cpu.set_register((opcode & 0x0F00) >> 8,cpu.get_register((opcode & 0x00F0) >> 4).unwrap()).unwrap();
                    cpu.set_program_counter(cpu.get_program_counter().unwrap() + 2).unwrap();
                },
                0x0001 =>{
                    cpu.set_register((opcode & 0x0F00) >> 8, cpu.get_register((opcode & 0x00F0) >> 4).unwrap() | cpu.get_register((opcode & 0x0F00) >> 8).unwrap()).unwrap();
                    cpu.set_program_counter(cpu.get_program_counter().unwrap() + 2).unwrap();
                },
                0x0002 =>{
                    cpu.set_register((opcode & 0x0F00) >> 8, cpu.get_register((opcode & 0x00F0) >> 4).unwrap() & cpu.get_register((opcode & 0x0F00) >> 8).unwrap()).unwrap();
                    cpu.set_program_counter(cpu.get_program_counter().unwrap() + 2).unwrap();
                },
                0x0003 =>{
                    cpu.set_register((opcode & 0x0F00) >> 8, cpu.get_register((opcode & 0x00F0) >> 4).unwrap() ^ cpu.get_register((opcode & 0x0F00) >> 8).unwrap()).unwrap();
                    cpu.set_program_counter(cpu.get_program_counter().unwrap() + 2).unwrap();
                },
                0x0004 =>{
                    if cpu.get_register((opcode & 0x00f0) >> 4).unwrap() > cpu.get_register((opcode & 0x0f00) >> 8).unwrap(){
                        cpu.set_register(0xF, 1).unwrap();
                    }else{
                        cpu.set_register(0xF, 0).unwrap();
                    }
                    cpu.set_register((opcode & 0x0f00) >> 8, cpu.get_register((opcode & 0x0f00) >> 8).unwrap() + cpu.get_register((opcode & 0x00f0) >> 4).unwrap()).unwrap();
                    cpu.set_program_counter(cpu.get_program_counter().unwrap() + 2).unwrap();
                    
                },

                0x0005 =>{
                    //set register to be subtracted values of VX and VY. Use VF as a carry, as each register can only hold 8 bits (max 255). If VF = 1, carry. else, do not.
                    if cpu.get_register((opcode & 0x00f0) >> 4).unwrap() > cpu.get_register((opcode & 0x0f00) >> 8).unwrap(){
                        cpu.set_register(0xF, 1).unwrap();
                    }else{
                        cpu.set_register(0xF, 0).unwrap();
                    }
                    cpu.set_register((opcode & 0x0f00) >> 8, cpu.get_register((opcode & 0x0f00) >> 8).unwrap() - cpu.get_register((opcode & 0x00f0) >> 4).unwrap()).unwrap();
                    cpu.set_program_counter(cpu.get_program_counter().unwrap() + 2).unwrap();
                    
                },

                0x0006 =>{
                    cpu.set_register(0xF, cpu.get_register((opcode & 0x0f00) >> 8).unwrap() >> 1).unwrap();
                    cpu.set_register((opcode & 0x0f00) >> 8,  cpu.get_register((opcode & 0x0f00) >> 8).unwrap() >> 1).unwrap();
                    cpu.set_program_counter(cpu.get_program_counter().unwrap() + 2).unwrap();
                },

                0x0007 =>{
                    //set register to be subtracted values of VX and VY. Use VF as a carry, as each register can only hold 8 bits (max 255). If VF = 1, carry. else, do not.
                    if cpu.get_register((opcode & 0x00f0) >> 4).unwrap() > cpu.get_register((opcode & 0x0f00) >> 8).unwrap(){
                        cpu.set_register(0xF, 1).unwrap();
                    }else{
                        cpu.set_register(0xF, 0).unwrap();
                    }
                    cpu.set_register((opcode & 0x0f00) >> 8, cpu.get_register((opcode & 0x00f0) >> 4).unwrap() - cpu.get_register((opcode & 0x0f00) >> 8).unwrap()).unwrap();
                    cpu.set_program_counter(cpu.get_program_counter().unwrap() + 2).unwrap();
                   
                },

                0x000E =>{
                    cpu.set_register(0xF, cpu.get_register((opcode & 0x0f00) >> 8).unwrap() >> 7).unwrap();
                    cpu.set_register((opcode & 0x0f00) >> 8,  cpu.get_register((opcode & 0x0f00) >> 8).unwrap() << 1).unwrap();
                    cpu.set_program_counter(cpu.get_program_counter().unwrap() + 2).unwrap();
                },

                _ => println!("Opcode {:#x?} is not implemented", opcode)
            }
        }

        0x9000..=0x9FFF => {
            if cpu.get_register((opcode & 0x0F00) >> 8).unwrap() != cpu.get_register((opcode & 0x00F0) >> 4).unwrap(){   
                cpu.set_program_counter(cpu.get_program_counter().unwrap() + 4).unwrap();
            }else{
                cpu.set_program_counter(cpu.get_program_counter().unwrap() + 2).unwrap();
            }
        },

        0xA000..=0xAFFF => {
            //take input of 0xANNN
            //set index register to NNN
            cpu.set_index_register(opcode & 0x0FFF).unwrap();
            //opcode = 2 instructions, so we move pc forward by 2
            cpu.set_program_counter(cpu.get_program_counter().unwrap() + 2).unwrap();
        
        },
        
        0xB000..=0xBFFF =>{
            cpu.set_program_counter((opcode & 0x0FFF) + cpu.get_register(0).unwrap() as u16).unwrap();
        },
        
        0xC000..=0xCFFF =>{
            cpu.set_register((opcode & 0x0F00) >> 8, ((0 % 0xFF) & (opcode & 0x00FF)) as u8).unwrap();
            cpu.set_program_counter(cpu.get_program_counter().unwrap() + 2).unwrap();
        }

        0xD000..=0xDFFF => {
            let x = cpu.get_register((opcode & 0x0F00) >> 8).unwrap();
            let y = cpu.get_register((opcode & 0x00F0) >> 4).unwrap();
            let height = opcode & 0x000F;
            let mut pixel = 0;

            cpu.set_register(0xF, 0).unwrap();
       
            for y_line in 0..height{
                pixel = cpu.get_memory(cpu.get_index_register().unwrap() + y_line).unwrap();
                for x_line in 0..8{
                    if (pixel & (0x80 >> x_line)) != 0{
                        if cpu.get_gfx(((x + x_line) as u16 + ((y as u16 + y_line) * 64)).into()).unwrap() == 1{
                            cpu.set_register(0xF, 1).unwrap();
                        }
                        cpu.set_gfx(((x as u16 + x_line as u16 + ((y as u16 + y_line) * 64)) % (64 * 32)).into(), cpu.get_gfx(((x as u16 + x_line as u16 + ((y as u16 + y_line) * 64)) % (64 * 32)).into()).unwrap() ^ 1).unwrap();
                        
                    }

                }
            }
            cpu.set_program_counter(cpu.get_program_counter().unwrap() + 2).unwrap();

        },
        0xE000..=0xEFFF => {
            match opcode & 0x00FF{
                0x009E => {
                    if cpu.get_key(cpu.get_register((opcode & 0x0F00) >> 8).unwrap()).unwrap() != 0{
                        cpu.set_program_counter(cpu.get_program_counter().unwrap() + 4).unwrap();
                    }else{
                        cpu.set_program_counter(cpu.get_program_counter().unwrap() + 2).unwrap();
                    }
                },

                0x00A1 => {
                    if cpu.get_key(cpu.get_register((opcode & 0x0F00) >> 8).unwrap()).unwrap() == 0{
                        cpu.set_program_counter(cpu.get_program_counter().unwrap() + 4).unwrap();
                    }else{
                        cpu.set_program_counter(cpu.get_program_counter().unwrap() + 2).unwrap();
                    }
                }

                _ => println!("Opcode {:#x?} is not implemented", opcode)
            }
        },
        
        0xF000..=0xFFFF =>{
            match opcode & 0x00FF{
                0x0007 => {
                    cpu.set_register((opcode & 0x0F00) >> 8, cpu.get_game_timer().unwrap().into()).unwrap();
                    cpu.set_program_counter(cpu.get_program_counter().unwrap() + 2).unwrap();
                },

                0x000A => {
                    let mut key_pressed = false;

                    for i in 0..16{
                        if cpu.get_key(i).unwrap() != 0{
                            key_pressed = true;
                            cpu.set_register((opcode & 0x0F00) >> 8, i).unwrap();
                        }
                    }
                    if key_pressed{
                        cpu.set_program_counter(cpu.get_program_counter().unwrap() + 2).unwrap();
                    }
                    
                }
                
                0x0015 => {
                    cpu.set_game_timer(cpu.get_register((opcode & 0x0F00) >> 8).unwrap()).unwrap();
                    cpu.set_program_counter(cpu.get_program_counter().unwrap() + 2).unwrap();
                },

                0x0018 => {
                    cpu.set_audio_timer(cpu.get_register((opcode & 0x0F00) >> 8).unwrap()).unwrap();
                    cpu.set_program_counter(cpu.get_program_counter().unwrap() + 2).unwrap();
                },

                0x001E => {
                    if cpu.get_index_register().unwrap() + cpu.get_register((opcode & 0x0F00) >> 8).unwrap() as u16 > 0x0FFF{
                        cpu.set_register(0xF, 1).unwrap();
                    }else{
                        cpu.set_register(0xF, 0).unwrap();
                    }
                    cpu.set_index_register(cpu.get_index_register().unwrap() + cpu.get_register((opcode & 0x0F00) >> 8).unwrap() as u16).unwrap();
                    cpu.set_program_counter(cpu.get_program_counter().unwrap() + 2).unwrap();
                },

                0x0029 => {
                    cpu.set_index_register((cpu.get_register((opcode & 0x0F00) >> 8).unwrap() * 0x5) as u16).unwrap();
                    cpu.set_program_counter(cpu.get_program_counter().unwrap() + 2).unwrap();
                }

                0x0033 => {
                    cpu.set_memory(cpu.get_register((opcode & 0x0F00) >> 8).unwrap() / 100, cpu.get_index_register().unwrap()).unwrap();
                    cpu.set_memory((cpu.get_register((opcode & 0x0F00) >> 8).unwrap() / 10) % 10, cpu.get_index_register().unwrap() + 1).unwrap();
                    cpu.set_memory((cpu.get_register((opcode & 0x0F00) >> 8).unwrap() % 100) % 10, cpu.get_index_register().unwrap() + 2).unwrap();
                    cpu.set_program_counter(cpu.get_program_counter().unwrap() + 2).unwrap();
                 
                },

                0x0055 => {
                    for i in 0..((opcode & 0x0F00) >> 8){
                        cpu.set_memory(cpu.get_register(i).unwrap(), cpu.get_index_register().unwrap() + i).unwrap();
                    }

                    cpu.set_index_register(cpu.get_index_register().unwrap() + cpu.get_register((opcode & 0x0F00) >> 8).unwrap() as u16 + 1).unwrap();
                    cpu.set_program_counter(cpu.get_program_counter().unwrap() + 2).unwrap();
                }

                0x0065 => {
                    for i in 0..((opcode & 0x0F00) >> 8){
                        cpu.set_register(i, cpu.get_memory(cpu.get_index_register().unwrap() + i).unwrap()).unwrap();
                    }

                    cpu.set_index_register(cpu.get_index_register().unwrap() + cpu.get_register((opcode & 0x0F00) >> 8).unwrap() as u16 + 1).unwrap();
                    cpu.set_program_counter(cpu.get_program_counter().unwrap() + 2).unwrap();
                }

                _ => println!("Opcode {:#x?} is not implemented", opcode)
            }


        },
        
        
        0x0004..=0xFFF4 => {
            
        },
        _ => println!("Opcode {:#x?} is not implemented", opcode)
    };

    let mut delay_timer = cpu.get_game_timer().unwrap();
    if delay_timer > 0{
        delay_timer -= 1;
        cpu.set_game_timer(delay_timer).unwrap();
    }

    let mut audio_timer = cpu.get_audio_timer().unwrap();
    if audio_timer >0{
        audio_timer -= 1;
        cpu.set_audio_timer(audio_timer).unwrap();
        if audio_timer == 1{
            println!("Beep Boop");
        }
    }

    Ok(cpu)
}