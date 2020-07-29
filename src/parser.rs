use byteorder::ReadBytesExt;
use std::io::Cursor;
use std::io::Read;
use wasabi_leb128::ReadLeb128;

pub struct Parser<'a> {
    cursor: Cursor<&'a [u8]>,
    len: usize,
}

impl<'a> Parser<'a> {
    pub fn new(buf: &'a [u8]) -> Parser<'a> {
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
            let (section_size, _): (u32, _) = self.cursor.read_leb128().unwrap();

            println!("section_id {}", section_id);
            println!("section_size: {}", section_size);

            match section_id {
                0 => {
                    // This is a custom section
                    unimplemented!("custom section");
                }
                1 => {
                    // This is the type section
                    let (type_vector_size, _) = self.cursor.read_leb128().unwrap();
                    for _ in 0..type_vector_size {
                        let func_byte = self.cursor.read_u8().unwrap();
                        assert_eq!(func_byte, 0x60, "Function did not start with 0x60");
                        let (param_vector_size, _): (u32, _) = self.cursor.read_leb128().unwrap();
                        let mut t: Type = Default::default();
                        for _ in 0..param_vector_size {
                            let param_value = self.cursor.read_u8().unwrap();
                            t.params.push(param_value);
                        }
                        let (return_vector_size, _): (u32, _) = self.cursor.read_leb128().unwrap();
                        for _ in 0..return_vector_size {
                            let return_value = self.cursor.read_u8().unwrap();
                            t.returns.push(return_value);
                        }
                        println!("{:?}", t);
                    }
                }
                2 => {
                    // This is the import section
                    unimplemented!("import section");
                }
                3 => {
                    let (func_vector_size, _): (u32, _) = self.cursor.read_leb128().unwrap();
                    for _ in 0..func_vector_size {
                        let (typeidx, _): (u32, _) = self.cursor.read_leb128().unwrap();
                        let func = Function {
                            type_index: typeidx,
                        };
                        println!("{:?}", func);
                    }
                }
                4 => {
                    // This is the table section
                    unimplemented!("table section");
                }
                5 => {
                    // This is the memory section
                    unimplemented!("memory section");
                }
                6 => {
                    // This is the global section
                    unimplemented!("global section");
                }
                7 => {
                    // This is the export section
                    let (export_vector_size, _): (u32, _) = self.cursor.read_leb128().unwrap();
                    for _ in 0..export_vector_size {
                        let (name_size, _): (u32, _) = self.cursor.read_leb128().unwrap();
                        let mut name_buf = vec![0u8; name_size as usize];
                        self.cursor.read_exact(&mut name_buf).unwrap();
                        let name = std::str::from_utf8(name_buf.as_slice());
                        let export_type = self.cursor.read_u8().unwrap();
                        let (export_index, _): (u32, _) = self.cursor.read_leb128().unwrap();

                        let export = Export {
                            name: name.unwrap().to_string(),
                            type_index: export_type,
                            index: export_index,
                        };
                        println!("{:?}", export);
                    }
                }
                8 => {
                    // This is the start section
                    unimplemented!("start section");
                }
                9 => {
                    // This is the element section
                    unimplemented!("element section");
                }
                10 => {
                    // This is the code section
                    println!("code section");
                    let (code_vector_size, _): (u32, _) = self.cursor.read_leb128().unwrap();
                    println!("code_vector_size: {}", code_vector_size);
                    for _ in 0..code_vector_size {
                        let (code_size, _): (u32, _) = self.cursor.read_leb128().unwrap();
                        let (locals_size, _): (u32, _) = self.cursor.read_leb128().unwrap();

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
                                    println!("local.get {}", localidx);
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
                    unimplemented!("data section");
                }
                _ => {
                    panic!("Bad section id");
                }
            }
        }
    }

    fn verify_preamble(&mut self) {
        let expected_preamble = vec![0x00, 0x61, 0x73, 0x6d];
        let expected_version = vec![0x01, 0x00, 0x00, 0x00];

        let mut preamble = vec![0u8; 4];
        let mut version = vec![0u8; 4];

        let res = self.cursor.read_exact(&mut preamble);
        if let Err(e) = res {
            panic!("Error reading preamble: {}", e);
        }

        let res = self.cursor.read_exact(&mut version);
        if let Err(e) = res {
            panic!("Error reading version: {}", e);
        }

        if preamble != expected_preamble {
            panic!("Invalid preamble!");
        }
        if version != expected_version {
            panic!("Invalid version!");
        }
    }
}

#[derive(Debug)]
struct Export {
    name: String,
    type_index: u8,
    index: u32,
}

#[derive(Debug)]
struct Function {
    type_index: u32,
}

#[derive(Debug, Default)]
struct Type {
    params: Vec<u8>,
    returns: Vec<u8>,
}
