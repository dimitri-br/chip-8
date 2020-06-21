pub mod cpu;
use cpu::{CPU, emulate_cycle, load_rom};


fn main(){
    println!("Hello!\nWelcome to the chip-8 emulator.\nIt currently isn't working :(\nPlease check later!");
    let mut cpu = CPU::new().unwrap();
    cpu = load_rom(cpu).unwrap();
    cpu.set_opcode(cpu.get_program_counter().unwrap()).unwrap();

    loop{
        cpu = emulate_cycle(cpu).unwrap();
    }  
}