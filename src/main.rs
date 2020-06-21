mod cpu;
use cpu::CPU;

fn main(){
    println!("Hello!\nWelcome to the chip-8 emulator.\nIt currently isn't working :(\nPlease check later!");
}



#[test]
fn test_cpu(){
    //this function will test the cpu is functioning as expected

    let value : u8 = 0x20; // set value to pass through

    //create cpu
    let mut cpu = CPU::new().expect("Error creating CPU instance");

    //write memory
    cpu.add_to_memory(value, 0).expect("Error writing to memory");
    //read memory
    let mem_value = cpu.read_from_memory(0).expect("Error reading from memory");

    //write register
    cpu.write_register(0xA, mem_value).expect("Error writing to register");
    //read register
    let reg_value = cpu.read_register(0xA).expect("Error reading from register");

    //write to stack
    cpu.write_subroutine(0x00, reg_value).expect("Error writing to stack");
    //read from stack
    let stack_value = cpu.read_subroutine(0x00).expect("Error reading from stack");

    //set timer
    cpu.set_timer(1, stack_value).expect("Error setting timer");


    //output the value
    println!("Audio timer: {:#?}", stack_value);
}
