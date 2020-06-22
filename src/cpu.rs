mod read;
use read::Reader;
use rand;
use rand::Rng;


pub const FONT_SET: [u8; 80] = [ 
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct CPU {
    pub pc : u16,
    pub opcode : u16,
    pub index_register : u16,
    pub sp : u8,
    pub registers : [u8; 16],
    pub memory : [u8; 4096],
    pub stack : [u16; 256],
    pub vram : [u8; 64 * 32],
    pub key : [u8; 16],
    pub audio_timer : u8,
    pub audio_play : bool,
    pub delay_timer : u8,
    pub draw : bool,
    mode: Mode,
    step : u64
}
pub enum Mode{
    Debug,
    Normal
}
pub fn load() -> CPU{
    let mut cpu = CPU { 
        pc: 0x200,
        opcode: 0x0,
        index_register: 0x0,
        sp: 0x0,
        registers : [0x0; 16],
        memory: [0x0; 4096],
        stack: [0x0; 256],
        vram: [0x0; 64 * 32],
        key: [0x0; 16],
        audio_timer: 0x0,
        audio_play: false,
        delay_timer: 0x0,
        draw: true,
        mode: Mode::Normal,
        step : 0,
    };
    for i in 0..80{
        cpu.memory[i + 0x0] = FONT_SET[i];
    }
    cpu
}
pub fn load_rom(mut cpu: CPU, file: String) -> CPU{
    let mut reader = Reader::new(file).unwrap();
    reader.open().unwrap();
    let mut current_mem = 0x200;
    for line in reader.ROM.iter(){
        cpu.memory[current_mem] = *line;
        current_mem += 1;
    }
    cpu
}


pub fn emulate_cycle(mut cpu : CPU) -> CPU{
    let opcode : u16 = (cpu.memory[cpu.pc as usize] as u16) << 8 | cpu.memory[(cpu.pc + 1) as usize]  as u16;
    let X = ((opcode & 0x0F00) >> 8) as usize;
    let Y = ((opcode & 0x00F0) >> 4) as usize;
    //println!("{:#x?}", opcode);
    match &cpu.mode{
        Mode::Debug => println!("DEBUG - Current Opcode: {:#x?}\nVX: {}\nVY: {}\nPC: {}\nI: {}\nSP: {}",&opcode,&cpu.registers[((&opcode & 0x0F00) >> 8) as usize], &cpu.registers[((&opcode & 0x00F0) >> 4) as usize], &cpu.pc, &cpu.index_register, &cpu.sp),
        _ => {}
    };
    match opcode & 0xF000{
        0x0000 => {
            match opcode & 0x00F0{
                0x0000 => {
                    cpu.vram = [0; 2048];
                    cpu.draw = true;
                    cpu.pc += 2;    
                }
                0x00E0 => {
                    cpu.sp -= 1;
                    cpu.pc = cpu.stack[cpu.sp as usize] as u16;
                    cpu.pc += 2;
                }
                _ => {}
            }
        }
        0x1000 => {
            cpu.pc = opcode & 0x0FFF;
            
        }
        0x2000 => {
            cpu.stack[cpu.sp as usize] = cpu.pc;
            cpu.sp += 1;
            cpu.pc = opcode & 0x0FFF;
        }
        0x3000 => {
            
            if (cpu.registers[X] as u16) == (opcode & 0x00FF){
                cpu.pc += 4;
                
                
            }else{
                cpu.pc += 2
            }
        }
        0x4000 => {
            if (cpu.registers[X] as u16) != (opcode & 0x00FF){
                cpu.pc += 4;
                
            }else{
                cpu.pc += 2
            }
        }
        0x5000 => {
            if cpu.registers[X] == cpu.registers[Y]{
                cpu.pc += 4;
                
            }else{
                cpu.pc += 2;
            }
        }
        0x6000 => {
            cpu.registers[X] = (opcode & 0x00FF) as u8;
            cpu.pc += 2;
            
        }
        0x7000 => {
            cpu.registers[X] += (opcode & 0x00FF) as u8;
            cpu.pc += 2;
            
        }
        0x8000 => {
           
            match opcode & 0x000F{
                    0x0000 =>{ // 0x8X Y0: Sets VX to the value of VY
                        cpu.registers[X] = cpu.registers[Y];

                        cpu.pc += 2; 
                    }
                    0x0001 =>{
                        cpu.registers[X] |= cpu.registers[Y];                      
                        cpu.pc += 2;
                    }
                    0x0002 =>{      
                        cpu.registers[X] &= cpu.registers[Y];
                        cpu.pc += 2;
                        
                    }
                    0x0003 =>{
                        cpu.registers[X] ^= cpu.registers[Y];
                        cpu.pc += 2;
                    }
                    0x0004 =>{
                        if cpu.registers[Y] > (0xFF - cpu.registers[X]){
                            cpu.registers[0xF] = 1;
                        }else{
                            cpu.registers[0xF] = 0;
                        }
                        cpu.registers[X] += cpu.registers[Y];
                        cpu.pc += 2;                        
                    }
                    0x0005 =>{
                        //set register to be subtracted values of VX and VY. Use VF as a carry, as each register can only hold 8 bits (max 255). If VF = 1, carry. else, do not.
                        if cpu.registers[Y] > cpu.registers[X]{
                            cpu.registers[0xF] = 0;
                        }else{
                            cpu.registers[0xF] = 1;
                        }
                        
                        cpu.registers[X] -= cpu.registers[Y];
                       
                        cpu.pc += 2;
                        
                    }
                    0x0006 =>{
                        
                        cpu.registers[0xF] = &cpu.registers[X] & 0x1;
                        cpu.registers[X] >>= 1;
                        
                        cpu.pc += 2;
                    }
                    0x0007 =>{
                        //set register to be subtracted values of VX and VY. Use VF as a carry, as each register can only hold 8 bits (max 255). If VF = 1, carry. else, do not.
                        if cpu.registers[X] > cpu.registers[Y]{
                            cpu.registers[0xF] = 0;
                        }else{
                            cpu.registers[0xF] = 1;
                        }
                        
                        cpu.registers[X] = &cpu.registers[Y] - &cpu.registers[X];
                        cpu.pc += 2;
                        
                       
                    }
                    0x000E =>{
                        cpu.registers[0xF] = &cpu.registers[X] >> 7;
                        cpu.registers[X] <<= 1;
                        cpu.pc += 2;
                    },
    
                    _ => {}
                }
            }
        0x9000 => {
            if cpu.registers[X] != cpu.registers[Y]{
				cpu.pc += 4;
            }else{
                cpu.pc += 2;
            }
        }
        0xA000 => {
            cpu.index_register = (opcode & 0x0FFF) as u16;
            cpu.pc += 2;
        }
        0xB000 => {
            cpu.pc = (opcode & 0x0FFF) as u16 + cpu.registers[0] as u16;
        }
        0xC000 => {
            cpu.registers[X] = ((rand::thread_rng().gen_range(0, 255) % 0xFF) & (opcode & 0x00FF)) as u8;
            cpu.pc += 2;
        }
        0xD000 => {
            let sprite_x = cpu.registers[X];
            let sprite_y = cpu.registers[Y];
            let sprite_height = opcode & 0x000F;

            cpu.registers[0xF] = 0;// Sets to 1 if there's a collision
       
            for y_line in 0..sprite_height{
                let mut line = cpu.memory[(cpu.index_register + y_line as u16) as usize];
                for x_line in 0..8{
                    let pixel = line & (0x80 >> x_line);
                    if pixel != 0{

                        

                       
                        
                        let idx = ((sprite_x as u16 + x_line as u16 + ((sprite_y as u16 + y_line) * 64)) % 2048) as usize;

                        
                        if cpu.vram[idx] != 0{
                            cpu.registers[0xF] = 1;
                        }
                        
                        cpu.vram[idx] ^= 1;
                        

                        
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
                    if cpu.key[cpu.registers[X] as usize] != 0{
                        cpu.pc += 4;
                    }else{
                        cpu.pc += 2;
                    }
                }
                0x00A1 => {
                    if cpu.key[cpu.registers[X] as usize] == 0{
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
                    cpu.registers[X] =  cpu.delay_timer;
                    cpu.pc += 2;
                }
                0x000A => {
                    let mut key_pressed = false;

                    for i in 0..16{
                        if cpu.key[i] != 0{
                            key_pressed = true;
                            cpu.registers[X] =  i as u8;
                        }
                    }
                    if key_pressed{
                        cpu.pc += 2;
                    }
                    
                }    
                0x0015 => {
                    cpu.delay_timer = cpu.registers[X];
                    cpu.pc += 2;
                }
                0x0018 => {
                    cpu.audio_timer = cpu.registers[X];
                    cpu.pc += 2;
                }
                0x001E => {
                    if cpu.index_register + cpu.registers[X] as u16 > 0x0FFF{
                        cpu.registers[0xF] = 1;
                    }else{
                        cpu.registers[0xF] = 0;
                    }
                    cpu.index_register += cpu.registers[X] as u16;
                    cpu.pc += 2;
                }
                0x0029 => {
                    cpu.index_register = 0x0 + (cpu.registers[X] * 0x5) as u16;
                    cpu.pc += 2;
                }
                0x0033 => {
                    cpu.memory[cpu.index_register as usize]     =  cpu.registers[X] / 100;
					cpu.memory[cpu.index_register as usize + 1] = (cpu.registers[X] / 10) % 10;
					cpu.memory[cpu.index_register as usize + 2] = (cpu.registers[X] % 100) % 10;
                    cpu.pc += 2;
                 
                }
                0x0055 => {
                    for i in 0..X + 1{
                        cpu.memory[cpu.index_register as usize + i] = cpu.registers[i];
                    }

                    cpu.index_register += X as u16 + 1;
                    cpu.pc += 2;
                }
                0x0065 => {
                    for i in 0..X + 1{
                        cpu.registers[i] = cpu.memory[cpu.index_register as usize + i];
                    }

                    cpu.index_register += X as u16 + 1;
                    cpu.pc += 2;
                }
                _ => {}
            }
    

        }
        _ => {panic!("Not known!")}


    }

    cpu.step += 1;

    if cpu.step % 2 == 0{
        if cpu.delay_timer > 0{
            cpu.delay_timer -= 1;
        } 
    }

    

    if cpu.audio_timer > 0{
        
        if cpu.audio_timer == 1{
            cpu.audio_play = true;
        }
        cpu.audio_timer -= 1;

    }
    cpu
}
