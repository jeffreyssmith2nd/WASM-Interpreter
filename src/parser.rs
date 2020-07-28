use byteorder::ReadBytesExt;
use std::io::Cursor;
use std::io::Read;
use wasabi_leb128::ReadLeb128;

pub struct Parser<'a> {
    cursor: Cursor<&'a [u8]>,
    len: usize,
}

impl<'a> Parser<'a> {
    pub fn new(buf: &'a Vec<u8>) -> Parser<'a> {
        Parser {
            len: buf.len(),
            cursor: Cursor::new(&buf),
        }
    }

    pub fn parse(&mut self) {
        self.verify_preamble();

        loop {
            if self.cursor.position() == self.len as u64 {
                break;
            }

            let section_id = self.cursor.read_u8().unwrap();
            println!("section_id {}", section_id);

            let (section_size, _): (u32, _) = self.cursor.read_leb128().unwrap();
            println!("section_size: {}", section_size);

            match section_id {
                0 => {
                    // This is a custom section
                    println!("custom section");
                    panic!("not implemented");
                }
                1 => {
                    // This is the type section
                    println!("type section");
                    let (type_vector_size, _) = self.cursor.read_leb128().unwrap();
                    println!("type_vector_size: {}", type_vector_size);
                    for _ in 0..type_vector_size {
                        let func_byte = self.cursor.read_u8().unwrap();
                        assert_eq!(func_byte, 0x60, "Function did not start with 0x60");
                        let (param_vector_size, _): (u32, _) = self.cursor.read_leb128().unwrap();
                        println!("param_vector_size: {}", param_vector_size);
                        for _ in 0..param_vector_size {
                            let param_value = self.cursor.read_u8().unwrap();
                            println!("par val: {:x}", param_value);
                        }
                        let (return_vector_size, _): (u32, _) = self.cursor.read_leb128().unwrap();
                        println!("return_vector_size: {}", return_vector_size);
                        for _ in 0..return_vector_size {
                            let return_value = self.cursor.read_u8().unwrap();
                            println!("ret val: {:x}", return_value);
                        }
                    }
                }
                2 => {
                    // This is the import section
                    println!("import section");
                    panic!("not implemented");
                }
                3 => {
                    println!("function section");
                    let (func_vector_size, _): (u32, _) = self.cursor.read_leb128().unwrap();
                    println!("func_vector_size: {}", func_vector_size);
                    for _ in 0..func_vector_size {
                        let (typeidx, _): (u32, _) = self.cursor.read_leb128().unwrap();
                        println!("typeidx: {}", typeidx);
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
                    let (export_vector_size, _): (u32, _) = self.cursor.read_leb128().unwrap();
                    println!("export_vector_size: {}", export_vector_size);
                    for _ in 0..export_vector_size {
                        let (name_size, _): (u32, _) = self.cursor.read_leb128().unwrap();
                        println!("name_size: {}", name_size);
                        let mut name_buf = vec![0u8; name_size as usize];
                        self.cursor.read_exact(&mut name_buf).unwrap();
                        let name = std::str::from_utf8(name_buf.as_slice());
                        println!("name {}", name.unwrap());
                        let export_type = self.cursor.read_u8().unwrap();
                        let (export_index, _): (u32, _) = self.cursor.read_leb128().unwrap();
                        println!("export_type {} export_index {}", export_type, export_index);
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
                    let (code_vector_size, _): (u32, _) = self.cursor.read_leb128().unwrap();
                    println!("code_vector_size: {}", code_vector_size);
                    for _ in 0..code_vector_size {
                        let (code_size, _): (u32, _) = self.cursor.read_leb128().unwrap();
                        println!("code_size: {}", code_size);
                        let (locals_size, _): (u32, _) = self.cursor.read_leb128().unwrap();
                        println!("locals_size: {}", locals_size);
                        for _ in 0..locals_size {
                            println!("do locals");
                        }
                        for _ in 0..code_size {
                            // Handle opcodes
                            let opcode = self.cursor.read_u8().unwrap();
                            println!("opcode: {:x}", opcode);
                            match opcode {
                                0x20 => {
                                    // local.get
                                    let localidx = self.cursor.read_u8().unwrap();
                                    println!("opcode: {:x}, localidx: {}", opcode, localidx);
                                }
                                0x0B => {
                                    // function end byte
                                    println!("function end byte");
                                    break;
                                }
                                0x6A => {
                                    // i32.add
                                    println!("i32.add");
                                }
                                _ => {
                                    println!("unmatched opcode");
                                }
                            }
                        }
                    }
                }
                11 => {
                    // This is the data section
                    println!("data section");
                    panic!("not implemented");
                }
                _ => {
                    panic!("Bad section id");
                }
            }
        }
    }

    pub fn verify_preamble(&mut self) {
        let expected_preamble = vec![0x00, 0x61, 0x73, 0x6d];
        let expected_version = vec![0x01, 0x00, 0x00, 0x00];

        let mut preamble = vec![0u8; 4];
        let mut version = vec![0u8; 4];

        self.cursor.read_exact(&mut preamble);
        self.cursor.read_exact(&mut version);

        if preamble != expected_preamble {
            panic!("Invalid preamble!");
        }
        if version != expected_version {
            panic!("Invalid version!");
        }
    }
}
