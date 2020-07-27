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

        loop {
            if self.buffer.len() == self.pos {
                break;
            }

            let section_id = self.buffer[self.pos];
            println!("section_id {}", section_id);
            self.pos += 1;

            let mut buf = std::io::Cursor::new(&self.buffer[self.pos..]);
            let (section_size, bytes_read): (u32, usize) = buf.read_leb128().unwrap();
            println!("section_size: {}", section_size);
            self.pos += bytes_read;

            match section_id {
                0 => {
                    // This is a custom section
                    println!("custom section");
                    panic!("not implemented");
                }
                1 => {
                    // This is the type section
                    println!("type section");
                    // TODO: Use one cursor
                    let mut buf = std::io::Cursor::new(&self.buffer[self.pos..]);
                    let (type_vector_size, bytes_read): (u32, usize) = buf.read_leb128().unwrap();
                    println!("type_vector_size: {}", type_vector_size);
                    self.pos += bytes_read;
                    for _ in 0..type_vector_size {
                        let func_byte = self.buffer[self.pos];
                        self.pos += 1;
                        assert_eq!(func_byte, 0x60, "Function did not start with 0x60");
                        let mut buf = std::io::Cursor::new(&self.buffer[self.pos..]);
                        let (param_vector_size, bytes_read): (u32, usize) =
                            buf.read_leb128().unwrap();
                        println!("param_vector_size: {}", param_vector_size);
                        self.pos += bytes_read;
                        let mut i: usize = 0;
                        while i < param_vector_size as usize {
                            println!("par val: {:x}", self.buffer[self.pos + i as usize]);
                            i += 1;
                        }
                        self.pos += i;
                        let mut buf = std::io::Cursor::new(&self.buffer[self.pos..]);
                        let (return_vector_size, bytes_read): (u32, usize) =
                            buf.read_leb128().unwrap();
                        println!("return_vector_size: {}", return_vector_size);
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
                    println!("import section");
                    panic!("not implemented");
                }
                3 => {
                    // This is the function section
                    println!("function section");
                    let mut buf = std::io::Cursor::new(&self.buffer[self.pos..]);
                    let (func_vector_size, bytes_read): (u32, usize) = buf.read_leb128().unwrap();
                    println!("func_vector_size: {}", func_vector_size);
                    self.pos += bytes_read;
                    for _ in 0..func_vector_size {
                        let mut buf = std::io::Cursor::new(&self.buffer[self.pos..]);
                        let (typeidx, bytes_read): (u32, usize) = buf.read_leb128().unwrap();
                        println!("typeidx: {}", typeidx);
                        self.pos += bytes_read;
                    }
                }
                4 => {
                    // This is the table section
                    println!("table section");
                    panic!("not implemented");
                }
                5 => {
                    // This is the memory section
                    println!("memory section");
                    panic!("not implemented");
                }
                6 => {
                    // This is the global section
                    println!("global section");
                    panic!("not implemented");
                }
                7 => {
                    // This is the export section
                    println!("export section");
                    let mut buf = std::io::Cursor::new(&self.buffer[self.pos..]);
                    let (export_vector_size, bytes_read): (u32, usize) = buf.read_leb128().unwrap();
                    println!("export_vector_size: {}", export_vector_size);
                    self.pos += bytes_read;
                    for _ in 0..export_vector_size {
                        let mut buf = std::io::Cursor::new(&self.buffer[self.pos..]);
                        let (name_size, bytes_read): (u32, usize) = buf.read_leb128().unwrap();
                        println!("name_size: {}", name_size);
                        self.pos += bytes_read;
                        let name = &self.buffer[self.pos..self.pos + name_size as usize];
                        let name = std::str::from_utf8(name);
                        println!("name {}", name.unwrap());
                        self.pos += name_size as usize;
                        let export_type = self.buffer[self.pos];
                        self.pos += 1;
                        let mut buf = std::io::Cursor::new(&self.buffer[self.pos..]);
                        let (export_index, bytes_read): (u32, usize) = buf.read_leb128().unwrap();
                        println!("export_type {} export_index {}", export_type, export_index);
                        self.pos += bytes_read;
                    }
                }
                8 => {
                    // This is the start section
                    println!("start section");
                    panic!("not implemented");
                }
                9 => {
                    // This is the element section
                    println!("element section");
                    panic!("not implemented");
                }
                10 => {
                    // This is the code section
                    println!("code section");
                    let mut buf = std::io::Cursor::new(&self.buffer[self.pos..]);
                    let (code_vector_size, bytes_read): (u32, usize) = buf.read_leb128().unwrap();
                    println!("code_vector_size: {}", code_vector_size);
                    self.pos += bytes_read;
                    for _ in 0..code_vector_size {
                        let mut buf = std::io::Cursor::new(&self.buffer[self.pos..]);
                        let (code_size, bytes_read): (u32, usize) = buf.read_leb128().unwrap();
                        println!("code_size: {}", code_size);
                        self.pos += bytes_read;
                        for _ in 0..code_size {
                            let mut buf = std::io::Cursor::new(&self.buffer[self.pos..]);
                            let (locals_size, bytes_read): (u32, usize) =
                                buf.read_leb128().unwrap();
                            println!("locals_size: {}", locals_size);
                            self.pos += bytes_read;
                            for _ in 0..locals_size {
                                println!("do locals");
                            }
                            // Handle opcodes
                        }
                    }
                }
                11 => {
                    // This is the data section
                    println!("data section");
                    panic!("not implemented");
                }
                _ => {
                    panic!("Invalid section encountered!");
                }
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
