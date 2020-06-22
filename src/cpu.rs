mod read;
use read::Reader;
use rand;
use rand::Rng;


const CHIP8_FONT : [u8; 5 * 16] =
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

pub struct CPU {
    pub pc : u16,
    pub opcode : u16,
    pub I : u16,
    pub sp : u8,
    pub V : [u8; 16],
    pub memory : [u8; 4096],
    pub stack : [u16; 256],
    pub vram : [u8; 64 * 32],
    pub key : [u8; 16],
    pub audio_timer : u8,
    pub delay_timer : u8,
    pub draw : bool,
}

pub fn load() -> CPU{
    let mut cpu = CPU { 
        pc: 0x200,
        opcode: 0x0,
        I: 0x0,
        sp: 0x0,
        V : [0x0; 16],
        memory: [0x0; 4096],
        stack: [0x0; 256],
        vram: [0x0; 64 * 32],
        key: [0x0; 16],
        audio_timer: 0x0,
        delay_timer: 0x0,
        draw: true,
    };
    for i in 0..80{
        cpu.memory[i] = CHIP8_FONT[i];
    }
    cpu
}
pub fn load_rom(mut cpu: CPU) -> CPU{
    let mut reader = Reader::new("programs/pong").unwrap();
    reader.open().unwrap();
    let mut current_mem = 0x200;
    for line in reader.content.iter(){
        cpu.memory[current_mem] = *line;
        current_mem += 1;
    }
    cpu
}
pub fn emulate_cycle(mut cpu : CPU) -> CPU{
    let opcode : u16 = (cpu.memory[cpu.pc as usize] as u16) << 8 | cpu.memory[(cpu.pc + 1) as usize]  as u16;
    //println!("{:#x?}", opcode);
    println!("DEBUG - Current Opcode: {:#x?}\nVX: {}\nVY: {}\nPC: {}\nI: {}\nSP: {}",opcode,cpu.V[((opcode & 0x0F00) >> 8) as usize], cpu.V[((opcode & 0x00F0) >> 4) as usize], cpu.pc, cpu.I, cpu.sp);
    match opcode & 0xF000{
        0x0000 => {
            match opcode & 0x00F0{
                0x0000 => {
                    for pixel in cpu.vram.iter_mut(){
                        *pixel = 0x0;
                    }
                    cpu.draw = true;
                    cpu.pc += 2;
                }
                0x00E0 => {
                    cpu.sp -= 1;
                    cpu.pc = cpu.stack[cpu.sp as usize] as u16;
                }
                _ => {}
            }
        }
        0x1000 => {
            cpu.pc = (opcode & 0x0FFF) as u16;
            
        }
        0x2000 => {
            cpu.stack[cpu.sp as usize] = cpu.pc + 2;
            cpu.sp += 1;
            cpu.pc = opcode & 0x0FFF;
        }
        0x3000 => {
            
            if cpu.V[((opcode & 0x0F00) >> 8) as usize] as u16 == (opcode & 0x00FF){
                cpu.pc += 4;
                
                
            }else{
                cpu.pc += 2
            }
        }
        0x4000 => {
            if cpu.V[((opcode & 0x0F00) >> 8) as usize]  as u16 != (opcode & 0x00FF){
                cpu.pc += 4;
                
            }else{
                cpu.pc += 2
            }
        }
        0x5000 => {
            if cpu.V[(((opcode & 0x0F00) >> 8) as usize) as usize] == cpu.V[(((opcode & 0x00F0) >> 4) as usize) as usize]{
                cpu.pc += 4;
                
            }else{
                cpu.pc += 2;
            }
        }
        0x6000 => {
            cpu.V[((opcode & 0x0F00) >> 8) as usize] = (opcode & 0x00FF) as u8;
            cpu.pc += 2;
            
        }
        0x7000 => {
            cpu.V[(((opcode & 0x0F00) >> 8) as usize) as usize] += (opcode & 0x00FF) as u8;
            cpu.pc += 2;
            
        }
        0x8000 => {
           
            match opcode & 0x000F{
                    0x0000 =>{ // 0x8X Y0: Sets VX to the value of VY
                        cpu.V[((opcode & 0x0F00) >> 8) as usize] = cpu.V[((opcode & 0x00F0) >> 4) as usize];
                        cpu.pc += 2;
                        
                    }
                    0x0001 =>{
                        cpu.V[((opcode & 0x0F00) >> 8) as usize] |= cpu.V[((opcode & 0x00F0) >> 4) as usize];
                        panic!("match! {} {}",cpu.V[(((opcode & 0x0F00) >> 8) as usize) as usize], cpu.V[(((opcode & 0x00f0) >> 4) as usize) as usize]);
                        
                        cpu.pc += 2;
                    }
                    0x0002 =>{
                        
                        cpu.V[((opcode & 0x0F00) >> 8) as usize] &= cpu.V[((opcode & 0x00F0) >> 4) as usize];
                        cpu.pc += 2;
                        
                    }
                    0x0003 =>{
                        cpu.V[((opcode & 0x0F00) >> 8) as usize] ^= cpu.V[((opcode & 0x00F0) >> 4) as usize];
                        cpu.pc += 2;
                    }
                    0x0004 =>{
                        //println!("VX before - {}",cpu.V[((opcode & 0x0F00) >> 8) as usize]);
                        if cpu.V[((opcode & 0x00F0) >> 4) as usize] > (0xFF - cpu.V[((opcode & 0x0F00) >> 8) as usize]){
                            cpu.V[0xF] = 1;
                        }else{
                            cpu.V[0xF] = 0;
                        }
                        cpu.V[((opcode & 0x0F00) >> 8) as usize] += cpu.V[((opcode & 0x00F0) >> 4) as usize];
                        cpu.pc += 2;
                        //panic!("VX: {} - VY: {} - V15: {}", cpu.V[((opcode & 0x0F00) >> 8) as usize], cpu.V[((opcode & 0x00F0) >> 4) as usize], cpu.V[0xF]);
                        
                    }
                    0x0005 =>{
                        //set register to be subtracted values of VX and VY. Use VF as a carry, as each register can only hold 8 bits (max 255). If VF = 1, carry. else, do not.
                        if cpu.V[((opcode & 0x00F0) >> 4) as usize] > cpu.V[((opcode & 0x0F00) >> 8) as usize]{
                            cpu.V[0xF] = 1;
                        }else{
                            cpu.V[0xF] = 0;
                        }
                        
                        cpu.V[((opcode & 0x0F00) >> 8) as usize] -= cpu.V[((opcode & 0x00F0) >> 4) as usize];
                       
                        cpu.pc += 2;
                        
                    }
                    0x0006 =>{
                        
                        cpu.V[0xF] =  cpu.V[((opcode & 0x0F00) >> 8) as usize] & 0x1;
                        cpu.V[((opcode & 0x0F00) >> 8) as usize] >>= 1;
                        
                        cpu.pc += 2;
                    }
                    0x0007 =>{
                        //set register to be subtracted values of VX and VY. Use VF as a carry, as each register can only hold 8 bits (max 255). If VF = 1, carry. else, do not.
                        if cpu.V[((opcode & 0x00F0) >> 4) as usize] > cpu.V[((opcode & 0x0F00) >> 8) as usize]{
                            cpu.V[0xF] = 1;
                        }else{
                            cpu.V[0xF] = 0;
                        }
                        
                        cpu.V[((opcode & 0x0F00) >> 8) as usize] = cpu.V[((opcode & 0x00F0) >> 4) as usize] - cpu.V[((opcode & 0x0F00) >> 8) as usize];
                        cpu.pc += 2;
                        
                       
                    }
                    0x000E =>{
                        cpu.V[0xF] = cpu.V[((opcode & 0x0F00) >> 8) as usize] >> 7;
                        cpu.V[((opcode & 0x0F00) >> 8) as usize] <<= 1;
                        cpu.pc += 2;
                    },
    
                    _ => {}
                }
            }
        0x9000 => {
            if cpu.V[((opcode & 0x0F00) >> 8) as usize] != cpu.V[((opcode & 0x00F0) >> 4) as usize]{
				cpu.pc += 4;
            }else{
                cpu.pc += 2;
            }
        }
        0xA000 => {
            cpu.I = (opcode & 0x0FFF) as u16;
            cpu.pc += 2;
        }
        0xB000 => {
            cpu.pc = (opcode & 0x0FFF) as u16 + cpu.V[0] as u16;
        }
        0xC000 => {
            cpu.V[((opcode & 0x0F00) >> 8) as usize] = ((rand::thread_rng().gen_range(0, 255) % 0xFF) & (opcode & 0x00FF)) as u8;
            cpu.pc += 2;
        }
        0xD000 => {
            let sprite_x = cpu.V[((opcode & 0x0F00) >> 8) as usize];
            let sprite_y = cpu.V[((opcode & 0x00F0) >> 4) as usize];
            let sprite_height = opcode & 0x000F;
            let mut pixel = 0;

            cpu.V[0xF] = 0;// Sets to 1 if there's a collision
       
            for y_line in 0..sprite_height{
                let line = cpu.memory[(cpu.I + y_line as u16) as usize];
                for x_line in 0..8{
                    pixel = line & (0x80 >> x_line);
                    if pixel != 0{

                        

                        let col = (sprite_x as u16 + x_line as u16) % 64;
                        let row = (sprite_y as u16 + y_line as u16) % 32;
                        
                        let idx = (col + (row * 64)) as usize;

                        let current_pixel = cpu.vram[idx];
                        cpu.vram[idx] ^= 1;
                        
                        
                        if current_pixel == 1{
                            cpu.V[0xF] = 1;
                        }

                        
                        //println!("old: {}\ncol: {}", current_pixel, cpu.V[0xF]);
                        //let pos_pixel = ((y as u16 + y_line) * 64 + (x as u16 + x_line as u16)) as usize;
                        /*if (y as u16 + y_line < 32) && (x as u16 + (x_line as u16) < 64){
                            cpu.vram[((y as u16 + y_line) * 64 + (x as u16 + x_line as u16)) as usize] ^= 1;
                        }
                        

                        let old_pixel_value = cpu.vram[pos_pixel % 2048];
                        


                        if cpu.vram[pos_pixel % 2048] == 1 && old_pixel_value != cpu.vram[pos_pixel % 2048]{
                            cpu.V[0xF] = 1;
                            
                        }*/
                        
                        
                    }

                }
            }
            cpu.draw = true;
            cpu.pc += 2;
        }
        0xE000 => {
            match opcode & 0x00FF{
                0x009E => {
                    if cpu.key[cpu.V[((opcode & 0x0F00) >> 8) as usize] as usize] != 0{
                        cpu.pc += 4;
                    }else{
                        cpu.pc += 2;
                    }
                }
                0x00A1 => {
                    if cpu.key[cpu.V[((opcode & 0x0F00) >> 8) as usize] as usize] == 0{
                        cpu.pc += 4;
                    }else{
                        cpu.pc += 2;
                    }
                }
                _ => {}
            }
        }
        0xF000 => {
            match opcode & 0x00FF{
                0x0007 => {
                    cpu.V[((opcode & 0x0F00) >> 8) as usize] =  cpu.delay_timer;
                    cpu.pc += 2;
                }
                0x000A => {
                    let mut key_pressed = false;

                    for i in 0..16{
                        if cpu.key[i] != 0{
                            key_pressed = true;
                            cpu.V[((opcode & 0x0F00) >> 8) as usize] =  i as u8;
                        }
                    }
                    if key_pressed{
                        cpu.pc += 2;
                    }
                    
                }    
                0x0015 => {
                    cpu.delay_timer = cpu.V[((opcode & 0x0F00) >> 8) as usize];
                    cpu.pc += 2;
                }
                0x0018 => {
                    cpu.audio_timer = cpu.V[((opcode & 0x0F00) >> 8) as usize];
                    cpu.pc += 2;
                }
                0x001E => {
                    if cpu.I + cpu.V[((opcode & 0x0F00) >> 8) as usize] as u16 > 0x0FFF{
                        cpu.V[0xF] = 1;
                    }else{
                        cpu.V[0xF] = 0;
                    }
                    cpu.I += cpu.V[((opcode & 0x0F00) >> 8) as usize] as u16;
                    cpu.pc += 2;
                }
                0x0029 => {
                    cpu.I = (cpu.V[((opcode & 0x0F00) >> 8) as usize] * 0x5) as u16;
                    cpu.pc += 2;
                }
                0x0033 => {
                    cpu.memory[cpu.I as usize]     = cpu.V[((opcode & 0x0F00) >> 8) as usize] / 100;
					cpu.memory[cpu.I as usize + 1] = (cpu.V[((opcode & 0x0F00) >> 8) as usize] / 10) % 10;
					cpu.memory[cpu.I as usize + 2] = (cpu.V[((opcode & 0x0F00) >> 8) as usize] % 100) % 10;
                    cpu.pc += 2;
                 
                }
                0x0055 => {
                    for i in 0..((((opcode & 0x0F00) >> 8) + 1) as usize){
                        cpu.memory[cpu.I as usize + i] = cpu.V[i];
                    }

                    cpu.I += ((opcode & 0x0F00 ) >> 8) + 1;
                    cpu.pc += 2;
                }
                0x0065 => {
                    for i in 0..((((opcode & 0x0F00) >> 8) + 1) as usize){
                        cpu.V[i] = cpu.memory[cpu.I as usize + i];
                    }

                    cpu.I += ((opcode & 0x0F00) >> 8) + 1;
                    cpu.pc += 2;
                }
                _ => {panic!("Unknown opcode: {:#x?}", opcode)}
            }
    

        }
        _ => {}


    }


    if cpu.delay_timer > 0{
        cpu.delay_timer -= 1;
    }

    if cpu.audio_timer > 0{
        if cpu.audio_timer == 1{
            println!("Beep! :)");
        }
        cpu.audio_timer -= 1;

    }
    cpu
}
