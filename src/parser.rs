use wasabi_leb128::ReadLeb128;

pub struct Parser {
    buffer: Vec<u8>,
    pos: usize,
}

impl Parser {
    pub fn new(buf: Vec<u8>) -> Parser {
        Parser {
            buffer: buf,
            pos: 0,
        }
    }

    pub fn parse(&mut self) {
        self.verify_preamble();

        let section_id = self.buffer[self.pos];
        println!("section_id {}", section_id);
        self.pos += 1;

        let mut buf = std::io::Cursor::new(&self.buffer[self.pos..]);
        let (section_size, bytes_read): (u32, usize) = buf.read_leb128().unwrap();
        println!("section_size: {} bytes_read {}", section_size, bytes_read);
        self.pos += bytes_read;

        match section_id {
            0 => {
                // This is a custom section
            }
            1 => {
                // This is the type section
                // TODO: Use one cursor
                let mut buf = std::io::Cursor::new(&self.buffer[self.pos..]);
                println!("{}", buf.position());
                let (type_vector_size, bytes_read): (u32, usize) = buf.read_leb128().unwrap();
                println!("{}", buf.position());
                println!(
                    "type_vector_size: {} bytes_read {}",
                    type_vector_size, bytes_read
                );
                self.pos += bytes_read;
                for _ in 0..type_vector_size {
                    let func_byte = self.buffer[self.pos];
                    self.pos += 1;
                    assert_eq!(func_byte, 0x60, "Function did not start with 0x60");
                    let mut buf = std::io::Cursor::new(&self.buffer[self.pos..]);
                    let (param_vector_size, bytes_read): (u32, usize) = buf.read_leb128().unwrap();
                    println!(
                        "param_vector_size: {} bytes_read {}",
                        param_vector_size, bytes_read
                    );
                    self.pos += bytes_read;
                    let mut i: usize = 0;
                    while i < param_vector_size as usize {
                        println!("par val: {:x}", self.buffer[self.pos + i as usize]);
                        i += 1;
                    }
                    self.pos += i;
                    let mut buf = std::io::Cursor::new(&self.buffer[self.pos..]);
                    let (return_vector_size, bytes_read): (u32, usize) = buf.read_leb128().unwrap();
                    println!(
                        "return_vector_size: {} bytes_read {}",
                        return_vector_size, bytes_read
                    );
                    self.pos += bytes_read;
                    let mut i: usize = 0;
                    while i < return_vector_size as usize {
                        println!("ret val: {:x}", self.buffer[self.pos + i as usize]);
                        i += 1;
                    }
                    self.pos += i;
                }
            }
            2 => {
                // This is the import section
            }
            3 => {
                // This is the function section
            }
            4 => {
                // This is the table section
            }
            5 => {
                // This is the memory section
            }
            6 => {
                // This is the global section
            }
            7 => {
                // This is the export section
            }
            8 => {
                // This is the start section
            }
            9 => {
                // This is the element section
            }
            10 => {
                // This is the code section
            }
            11 => {
                // This is the data section
            }
            _ => {
                panic!("Invalid section encountered!");
            }
        }
    }

    pub fn verify_preamble(&mut self) {
        let preamble: Vec<u8> = vec![0x00, 0x61, 0x73, 0x6d];
        if preamble[..] != self.buffer[0..4] {
            panic!("Invalid preamble!");
        }
        let version: Vec<u8> = vec![0x01, 0x00, 0x00, 0x00];
        if version[..] != self.buffer[4..8] {
            panic!("Invalid version!");
        }
        self.pos = 8;
    }
}
