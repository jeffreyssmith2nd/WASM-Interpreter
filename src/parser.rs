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

        let mut m: Module = Default::default();

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
                        m.types.push(t);
                    }
                }
                2 => {
                    // This is the import section
                    let (import_vector_size, _): (u32, _) = self.cursor.read_leb128().unwrap();
                    for _ in 0..import_vector_size {
                        let (module_name_size, _): (u32, _) = self.cursor.read_leb128().unwrap();
                        let module_name = self.read_string(module_name_size as usize);

                        let (name_size, _): (u32, _) = self.cursor.read_leb128().unwrap();
                        let name = self.read_string(name_size as usize);

                        let import_type = self.cursor.read_u8().unwrap();
                        match import_type {
                            0x00 => {
                                println!("found a functype import");
                                let (func_index, _): (u32, _) = self.cursor.read_leb128().unwrap();
                                println!("{:?}", func_index);
                            }
                            _ => panic!("umatched"),
                        }

                        let import = Import {
                            module: module_name,
                            name: name,
                            import_type: import_type,
                        };
                        println!("{:?}", import);
                        m.imports.push(import);
                    }
                }
                3 => {
                    let (func_vector_size, _): (u32, _) = self.cursor.read_leb128().unwrap();
                    for _ in 0..func_vector_size {
                        let (typeidx, _): (u32, _) = self.cursor.read_leb128().unwrap();
                        let func = Function {
                            type_index: typeidx,
                        };
                        println!("{:?}", func);
                        m.functions.push(func);
                    }
                }
                4 => {
                    // This is the table section
                    let (table_vector_size, _): (u32, _) = self.cursor.read_leb128().unwrap();
                    for _ in 0..table_vector_size {
                        let min: u32;
                        let max: u32;

                        let elem_type = self.cursor.read_u8().unwrap();
                        assert_eq!(elem_type, 0x70, "Elem type was not 0x70");
                        let limits_flag = self.cursor.read_u8().unwrap();
                        match limits_flag {
                            0x00 => {
                                let (m, _): (u32, _) = self.cursor.read_leb128().unwrap();
                                min = m;
                                max = 0;
                            }
                            0x01 => {
                                let (mi, _): (u32, _) = self.cursor.read_leb128().unwrap();
                                let (ma, _): (u32, _) = self.cursor.read_leb128().unwrap();
                                min = mi;
                                max = ma;
                            }
                            _ => {
                                panic!("Incorrect table limit flag");
                            }
                        }
                        let table = Table { min: min, max: max };
                        println!("{:?}", table);
                        m.tables.push(table);
                    }
                }
                5 => {
                    // This is the memory section
                    let (memory_vector_size, _): (u32, _) = self.cursor.read_leb128().unwrap();
                    for _ in 0..memory_vector_size {
                        let min: u32;
                        let max: u32;

                        let limits_flag = self.cursor.read_u8().unwrap();
                        match limits_flag {
                            0x00 => {
                                let (m, _): (u32, _) = self.cursor.read_leb128().unwrap();
                                min = m;
                                max = 0;
                            }
                            0x01 => {
                                let (mi, _): (u32, _) = self.cursor.read_leb128().unwrap();
                                let (ma, _): (u32, _) = self.cursor.read_leb128().unwrap();
                                min = mi;
                                max = ma;
                            }
                            _ => {
                                panic!("Incorrect table limit flag");
                            }
                        }
                        let memory = Memory { min: min, max: max };
                        println!("{:?}", memory);
                        m.memories.push(memory);
                    }
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
                        let name = self.read_string(name_size as usize);

                        let export_type = self.cursor.read_u8().unwrap();
                        let (export_index, _): (u32, _) = self.cursor.read_leb128().unwrap();

                        let export = Export {
                            name: name,
                            type_index: export_type,
                            index: export_index,
                        };
                        println!("{:?}", export);
                        m.exports.push(export);
                    }
                }
                8 => {
                    // This is the start section
                    unimplemented!("start section");
                }
                9 => {
                    // This is the element section
                    let (elem_vector_size, _): (u32, _) = self.cursor.read_leb128().unwrap();
                    for _ in 0..elem_vector_size {
                        let (table_idx, _): (u32, _) = self.cursor.read_leb128().unwrap();
                        self.match_opcodes_size(2); // For opcode and function end byte
                        let (func_idx_size, _): (u32, _) = self.cursor.read_leb128().unwrap();
                        println!("func_idx_size {}", func_idx_size);
                        for _ in 0..func_idx_size {
                            let (func_idx, _): (u32, _) = self.cursor.read_leb128().unwrap();
                        }
                    }
                }
                10 => {
                    // This is the code section
                    println!("code section");
                    let (code_vector_size, _): (u32, _) = self.cursor.read_leb128().unwrap();
                    println!("code_vector_size: {}", code_vector_size);
                    for i in 0..code_vector_size {
                        println!("\nFUNCTION {}", i);
                        let (code_size, _): (u32, _) = self.cursor.read_leb128().unwrap();
                        let (locals_size, _): (u32, _) = self.cursor.read_leb128().unwrap();

                        for _ in 0..locals_size {
                            let local_type = self.cursor.read_u8().unwrap();
                            match local_type {
                                0x7F | 0x7E | 0x7D | 0x7C => {}
                                _ => {
                                    panic!("invalid local type: {}", local_type);
                                }
                            }
                            print!("\tlocal_type: {:x}", local_type);
                        }

                        self.match_opcodes();
                        println!("\n");
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

    fn read_string(&mut self, size: usize) -> String {
        let mut buf = vec![0u8; size as usize];
        self.cursor.read_exact(&mut buf).unwrap();
        let s = std::str::from_utf8(buf.as_slice());
        return s.unwrap().to_string();
    }

    fn match_opcode(&mut self) -> bool {
        let opcode = self.cursor.read_u8().unwrap();
        // println!("opcode: {:x}", opcode);
        match opcode {
            0x00 => {
                panic!("unreachable");
            }
            0x01 => {
                // nop
                println!("nop");
            }
            0x02 => {
                // block
                let block_type = self.cursor.read_u8().unwrap();
                println!("block {:x}", block_type);
                self.match_opcodes();
            }
            0x04 => {
                // if
                let block_type = self.cursor.read_u8().unwrap();
                // TODO: how to handle block_type??
                println!("if {:x}", block_type);
                self.match_opcodes();
                /*
                let nb = self.peek_byte();
                println!("nb {:x}", nb);
                if nb == 0x05 {
                    panic!("need to handle else case");
                }
                */
            }
            0x05 => {
                // else
                println!("else");
                // TODO: how to handle block_type??
                self.match_opcodes();
            }
            0x0B => {
                // function end byte
                println!("function end byte");
                return true;
            }
            0x0D => {
                // br_if
                let (label_idx, _): (u32, _) = self.cursor.read_leb128().unwrap();
                println!("label_idx: {}", label_idx);
            }
            0x0E => {
                // br_table
                print!("br_table: ");
                let (label_size, _): (u32, _) = self.cursor.read_leb128().unwrap();
                print!("label_size: {}", label_size);
                for _ in 0..label_size {
                    let (label_idx, _): (u32, _) = self.cursor.read_leb128().unwrap();
                    print!("\tlabel_idx: {}", label_idx);
                }
                let (label_idx, _): (u32, _) = self.cursor.read_leb128().unwrap();
                println!(" label_idx: {}", label_idx);
            }
            0x10 => {
                // call
                let (func_idx, _): (u32, _) = self.cursor.read_leb128().unwrap();
                println!("call {}", func_idx);
            }
            0x20 => {
                // local.get
                let local_idx = self.cursor.read_u8().unwrap();
                println!("local.get {}", local_idx);
            }
            0x2D => {
                // i32.load8_u
                let align = self.cursor.read_u8().unwrap();
                let offset = self.cursor.read_u8().unwrap();
                println!("i32.load8_u {} {}", align, offset);
            }
            0x36 => {
                // i32.store
                let align = self.cursor.read_u8().unwrap();
                let offset = self.cursor.read_u8().unwrap();
                println!("i32.store {} {}", align, offset);
            }
            0x41 => {
                // i32.const
                let (val, _): (u32, _) = self.cursor.read_leb128().unwrap();
                println!("i32.const: {}", val);
            }
            0x6A => {
                // i32.add
                println!("i32.add");
            }
            0x6C => {
                // i32.mul
                println!("i32.mul");
            }
            0x71 => {
                //i32.and
                println!("i32.and");
            }
            _ => {
                panic!("unmatched opcode: {:x}", opcode);
            }
        }
        false
    }

    fn match_opcodes(&mut self) {
        loop {
            let br = self.match_opcode();
            if br {
                break;
            }
        }
    }

    fn match_opcodes_size(&mut self, code_size: u32) {
        for _ in 0..code_size {
            self.match_opcode();
        }
    }

    fn peek_byte(&mut self) -> u8 {
        let pos = self.cursor.position();
        let val = self.cursor.read_u8().unwrap();
        self.cursor.set_position(pos);
        val
    }
}

#[derive(Debug)]
struct Export {
    name: String,
    type_index: u8,
    index: u32,
}

#[derive(Debug)]
struct Import {
    module: String,
    name: String,
    import_type: u8,
}

#[derive(Debug)]
struct Function {
    type_index: u32,
}

#[derive(Debug)]
struct Table {
    min: u32,
    max: u32,
}

#[derive(Debug)]
struct Memory {
    min: u32,
    max: u32,
}

#[derive(Debug, Default)]
struct Type {
    params: Vec<u8>,
    returns: Vec<u8>,
}

#[derive(Default)]
struct Module {
    imports: Vec<Import>,
    types: Vec<Type>,
    tables: Vec<Table>,
    memories: Vec<Memory>,
    functions: Vec<Function>,
    exports: Vec<Export>,
}
