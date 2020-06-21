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

    match opcode & 0xF000{
        0x2..=0x2FFF =>{
            //this opcode runs a subroutine
            let sp = cpu.get_stack_pointer().unwrap(); //get current stack pointer
            cpu.write_subroutine(sp, cpu.get_program_counter().unwrap()).unwrap(); //save our current address to the stack
            if sp > 10{
                cpu.set_stack_pointer(0).unwrap(); //set new stack pointer to 0 so we dont get an IOoB error
            }else{
                cpu.set_stack_pointer(sp + 1).unwrap(); //set new stack pointer + 1 so we don't overwrite
            }
            
            cpu.set_program_counter(opcode & 0x0FFF).unwrap(); //set PC to new subroutine location
            println!("Run subroutine");
        },
        0x0..=0x0FFF => {
            
            match opcode & 0x000F{
                0x0000 => println!("Clear screen!"), //0x00E0
                0x000E => println!("Return subroutine"), //0x00EE
                _ => println!("Unknown opcode: {:#x?}",opcode),

            }
        },
        0xA..=0xAFFF => {
            //take input of 0xANNN
            //set index register to NNN
            cpu.set_index_register(opcode & 0x0FFF).unwrap();
            //opcode = 2 instructions, so we move pc forward by 2
            cpu.set_program_counter(cpu.get_program_counter().unwrap() + 2).unwrap();
            println!("Jump index register");
        },
        
        
        
        0xD..=0xDFFF => {
            let x = cpu.get_register((opcode & 0x0F00) >> 8).unwrap();
            let y = cpu.get_register((opcode & 0x00F0) >> 4).unwrap();
            let height = opcode & 0x000F;
            let mut pixel : u8 = 0;

            cpu.set_register(0xF, 0).unwrap();
            println!("Rendering!");
            for y_line in 0..height{
                pixel = cpu.get_memory(cpu.get_index_register().unwrap() + y_line).unwrap();
                for x_line in 0..8{
                    if (pixel & (0x80 >> x_line)) != 0{
                        if cpu.get_gfx(((x + x_line) as u16 + ((y as u16 + y_line) * 64)).into()).unwrap() == 1{
                            cpu.set_register(0xF, 1).unwrap();
                        }
                        cpu.set_gfx(((x as u16 + x_line as u16 + ((y as u16 + y_line) * 64)) % (64 * 32)).into(), 0).unwrap();
                        
                    }

                }
            }

        },
        0xE..=0xEFFF => {
            println!("Handle user input here!");
        },
        
        0x0..=0xFF33 => {
            cpu.set_memory(cpu.get_register((opcode & 0x0F00) >> 8).unwrap() / 100, cpu.get_index_register().unwrap()).unwrap();
            cpu.set_memory((cpu.get_register((opcode & 0x0F00) >> 8).unwrap() / 10) % 10, cpu.get_index_register().unwrap() + 1).unwrap();
            cpu.set_memory((cpu.get_register((opcode & 0x0F00) >> 8).unwrap() % 100) % 10, cpu.get_index_register().unwrap() + 2).unwrap();
            cpu.set_program_counter(cpu.get_program_counter().unwrap() + 2).unwrap();
            println!("Store VX in memory")
        },
        0x0..=0xFFF4 => {
            //set register to be added values of VX and VY. Use VF as a carry, as each register can only hold 8 bits (max 255). If VF = 1, carry. else, do not.
            if cpu.get_register((opcode & 0x00f0) >> 4).unwrap() > cpu.get_register((opcode & 0x0f00) >> 8).unwrap(){
                cpu.set_register(0xF, 1).unwrap();
            }else{
                cpu.set_register(0xF, 0).unwrap();
            }
            cpu.set_register((opcode & 0x0f00) >> 8, cpu.get_register((opcode & 0x0f00) >> 8).unwrap() + cpu.get_register((opcode & 0x00f0) >> 4).unwrap()).unwrap();
            cpu.set_program_counter(cpu.get_program_counter().unwrap() + 2).unwrap();
            println!("Add VX and VY");
        },
        _ => println!("Opcode {:#x?} has not been implemented yet",opcode),
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