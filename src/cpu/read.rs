use std::fs::File;
use std::io::Read;

pub struct Reader{
    file : String,
    pub ROM : [u8; 3584]
}

impl Reader{
    pub fn new(path : String) -> Result<Self, &'static str>{
        
        let reader = Reader { file : path.to_owned(), ROM: [0x0; 3584]};    
        Ok(reader)


    }
    pub fn open(&mut self) -> Result<(), &'static str>{
        let mut file = File::open(&self.file).expect("Error opening file");
        let mut buffer = [0u8; 3584];
        let mut len = 0;
        let _bytes_read = if let Ok(bytes_read) = file.read(&mut buffer) {
            len += 1;
            bytes_read
        } else {
            0
        };
        let mut mem = 0;
        for _ in buffer.iter(){
            mem += 1;
        }
        if mem > 4096 - 512{
            panic!("Error! Max file size is {} bytes, but ROM loaded was {} bytes", 4096 - 512, len)
        }
        self.ROM = buffer;
        
        println!("• Loaded ROM: {}", self.file);
       
        Ok(())
    }
    
}