use std::fs::File;
use std::io::Read;

pub struct Reader{
    file : String,
    pub content : Vec::<u8>
}

impl Reader{
    pub fn new(path : String) -> Result<Self, &'static str>{
        
        let reader = Reader { file : path.to_owned(), content : Vec::<u8>::new()};    
        Ok(reader)


    }
    pub fn open(&mut self) -> Result<(), &'static str>{
        let mut file = File::open(&self.file).expect("Error opening file");
        file.read_to_end(&mut self.content).expect("Error reading file");
        let len = self.content.len();
        if len > 4096 - 512{
            panic!("Error! Max file size is {} bytes, but ROM loaded was {} bytes", 4096 - 512, len)
        }
        println!("Loaded ROM: {}\nROM size:{}", self.file, len);
        Ok(())
    }
}