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
        cpu.add_to_memory(*line, current_mem).unwrap();
        current_mem += 1;
    }
    cpu.set_opcode(cpu.get_program_counter().unwrap()).unwrap();

    println!("{:#x?}", cpu.read_index_register().unwrap());
    for _ in 0..256{
        cpu = emulate_cycle(cpu).unwrap();
        println!("{:#x?}", cpu.read_index_register().unwrap());
    }
    

}


fn emulate_cycle(mut cpu : CPU) -> Result<CPU, &'static str>{
    cpu.set_opcode(cpu.get_program_counter().unwrap()).unwrap();
    let opcode = &cpu.get_opcode().unwrap();
    match opcode & 0xF000{
        0xA..=0xAFFF => {
            cpu.set_index_register(opcode & 0x0FFF).unwrap();
            cpu.set_program_counter(cpu.get_program_counter().unwrap() + 2).unwrap();
        },
        0x000 => {
            
            match opcode & 0x000F{
                0x0000 => println!("Clear screen!"), //0x00E0
                0x000E => println!("Return subroutine"), //0x00EE
                _ => println!("Unknown opcode: {:#x?}",opcode),

            }
        },
        0x2..=0x2FFF =>{
            let sp = cpu.get_stack_pointer().unwrap();
            cpu.write_subroutine(sp, cpu.get_program_counter().unwrap()).unwrap();
            cpu.set_stack_pointer(sp + 1).unwrap();
            cpu.set_program_counter(opcode & 0x0FFF).unwrap();
        },
        0x0004 => {
            if cpu.get_register((opcode & 0x00f0) >> 4).unwrap() > cpu.get_register((opcode & 0x0f00) >> 8).unwrap(){
                cpu.set_register(0xF, 1).unwrap();
            }else{
                cpu.set_register(0xF, 0).unwrap();
            }
            cpu.set_register((opcode & 0x0f00) >> 8, cpu.get_register((opcode & 0x0f00) >> 8).unwrap() + cpu.get_register((opcode & 0x00f0) >> 4).unwrap()).unwrap();
            cpu.set_program_counter(cpu.get_program_counter().unwrap() + 2).unwrap();
        }
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