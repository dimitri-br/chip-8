mod cpu;
mod read;
use cpu::CPU;
use read::Reader;

fn main(){
    println!("Hello!\nWelcome to the chip-8 emulator.\nIt currently isn't working :(\nPlease check later!");
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

    loop{
        cpu = emulate_cycle(cpu).unwrap();

    }
    

}


fn emulate_cycle(mut cpu : CPU) -> Result<CPU, &'static str>{
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