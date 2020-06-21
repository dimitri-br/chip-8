use std::fs::File;
use std::io::Read;

pub struct Reader{
    file : &'static str,
    pub content : Vec::<u8>
}

impl Reader{
    pub fn new(path : &'static str) -> Result<Self, &'static str>{
        let reader = Reader { file : path, content : Vec::<u8>::new()};    
        Ok(reader)


    }
    pub fn open(&mut self) -> Result<(), &'static str>{
        let mut file = File::open(self.file).expect("Error opening file");
        file.read_to_end(&mut self.content).expect("Error reading file");
        Ok(())
    }
}