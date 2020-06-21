mod cpu;
use cpu::CPU;

fn main(){
    //create cpu
    let mut cpu = CPU::new().expect("Error creating CPU instance");

    //]write memory
    cpu.add_to_memory(0x20, 0).expect("Error writing to memory");
    //read memory
    let mem_value = cpu.read_from_memory(0).expect("Error reading from memory");

    //write register
    cpu.write_register(0xA, mem_value).expect("Error writing to register");
    //read register
    let reg_value = cpu.read_register(0xA).expect("Error reading from register");

    cpu.write_subroutine(0x00, reg_value).expect("Error writing to stack");
    //output the value
    println!("{:#x?}",reg_value);
}
