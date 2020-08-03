use byteorder::ReadBytesExt;
use num_traits::{AsPrimitive, PrimInt};
use std::io::Cursor;
use std::io::Read;

use wasabi_leb128::ReadLeb128;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(FromPrimitive)]
enum Section {
    Custom = 0,
    Type,
    Import,
    Function,
    Table,
    Memory,
    Global,
    Export,
    Start,
    Element,
    Code,
    Data,
}

#[derive(FromPrimitive)]
enum ValueType {
    I32 = 0x7F,
    I64 = 0x7E,
    F32 = 0x7D,
    F64 = 0x7C,
}

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

            let section_id = self.get_u8();
            let section_size = self.get_u32();

            println!("section_id {}", section_id);
            println!("section_size: {}", section_size);

            match FromPrimitive::from_u8(section_id) {
                Some(Section::Custom) => {
                    self.cursor
                        .set_position(self.cursor.position() + section_size as u64);
                }
                Some(Section::Type) => {
                    let type_vector_size = self.get_u32();
                    for _ in 0..type_vector_size {
                        let func_byte = self.get_u8();
                        assert_eq!(func_byte, 0x60, "Function did not start with 0x60");

                        let param_vector_size = self.get_u32();
                        let mut t: Type = Default::default();
                        for _ in 0..param_vector_size {
                            let param_value = self.get_u8();
                            t.params.push(param_value);
                        }
                        let return_vector_size = self.get_u32();
                        for _ in 0..return_vector_size {
                            let return_value = self.get_u8();
                            t.returns.push(return_value);
                        }
                        println!("{:?}", t);
                        m.types.push(t);
                    }
                }
                Some(Section::Import) => {
                    let import_vector_size = self.get_u32();
                    for _ in 0..import_vector_size {
                        let module_name_size = self.get_u32();
                        let module_name = self.read_string(module_name_size as usize);

                        let name_size = self.get_u32();
                        let name = self.read_string(name_size as usize);

                        let import_type = self.get_u8();
                        match import_type {
                            0x00 => {
                                println!("found a functype import");
                                let func_index = self.get_u32();
                                println!("{:?}", func_index);
                            }
                            _ => panic!("umatched"),
                        }

                        let import = Import {
                            module: module_name,
                            name,
                            import_type,
                        };
                        println!("{:?}", import);
                        m.imports.push(import);
                    }
                }
                Some(Section::Function) => {
                    let func_vector_size = self.get_u32();
                    for _ in 0..func_vector_size {
                        let type_idx = self.get_u32();
                        let func = Function {
                            type_index: type_idx,
                        };
                        println!("{:?}", func);
                        m.functions.push(func);
                    }
                }
                Some(Section::Table) => {
                    let table_vector_size = self.get_u32();
                    for _ in 0..table_vector_size {
                        let min: u32;
                        let max: u32;

                        let elem_type = self.get_u8();
                        assert_eq!(elem_type, 0x70, "Elem type was not 0x70");
                        let limits_flag = self.get_u8();
                        match limits_flag {
                            0x00 => {
                                let m = self.get_u32();
                                min = m;
                                max = 0;
                            }
                            0x01 => {
                                let mi = self.get_u32();
                                let ma = self.get_u32();
                                min = mi;
                                max = ma;
                            }
                            _ => {
                                panic!("Incorrect table limit flag");
                            }
                        }
                        let table = Table { min, max };
                        println!("{:?}", table);
                        m.tables.push(table);
                    }
                }
                Some(Section::Memory) => {
                    let memory_vector_size = self.get_u32();
                    for _ in 0..memory_vector_size {
                        let min: u32;
                        let max: u32;

                        let limits_flag = self.get_u8();
                        match limits_flag {
                            0x00 => {
                                let m = self.get_u32();
                                min = m;
                                max = 0;
                            }
                            0x01 => {
                                let mi = self.get_u32();
                                let ma = self.get_u32();
                                min = mi;
                                max = ma;
                            }
                            _ => {
                                panic!("Incorrect table limit flag");
                            }
                        }
                        let memory = Memory { min, max };
                        println!("{:?}", memory);
                        m.memories.push(memory);
                    }
                }
                Some(Section::Global) => {
                    unimplemented!("global section");
                }
                Some(Section::Export) => {
                    let export_vector_size = self.get_u32();
                    for _ in 0..export_vector_size {
                        let name_size = self.get_u32();
                        let name = self.read_string(name_size as usize);

                        let export_type = self.get_u8();
                        let export_index = self.get_u32();

                        let export = Export {
                            name,
                            type_index: export_type,
                            index: export_index,
                        };
                        println!("{:?}", export);
                        m.exports.push(export);
                    }
                }
                Some(Section::Start) => {
                    unimplemented!("start section");
                }
                Some(Section::Element) => {
                    let elem_vector_size = self.get_u32();
                    for _ in 0..elem_vector_size {
                        let table_idx = self.get_u32();
                        self.match_opcodes_size(2); // For opcode and function end byte
                        let func_idx_size = self.get_u32();
                        println!("func_idx_size {}", func_idx_size);
                        for _ in 0..func_idx_size {
                            let func_idx = self.get_u32();
                        }
                    }
                }
                Some(Section::Code) => {
                    println!("code section");
                    let code_vector_size = self.get_u32();
                    println!("code_vector_size: {}", code_vector_size);
                    for i in 0..code_vector_size {
                        let code_size = self.get_u32();
                        let local_vector_size = self.get_u32();
                        println!("\nFUNCTION {}\tcode_size: {}", i, code_size);

                        for _ in 0..local_vector_size {
                            let num_locals = self.get_u32();
                            let local_type = self.get_u8();
                            match FromPrimitive::from_u8(local_type) {
                                Some(ValueType::I32) | Some(ValueType::I64)
                                | Some(ValueType::F32) | Some(ValueType::F64) => {}
                                None => {
                                    panic!("invalid local type: {}", local_type);
                                }
                            }
                            println!("\tnum_locals: {}, local_type: {:x}", num_locals, local_type);
                        }

                        self.match_opcodes();
                        println!("\n");
                    }
                }
                Some(Section::Data) => {
                    // This is the data section
                    unimplemented!("data section");
                }
                None => {
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
        s.unwrap().to_string()
    }

    fn match_opcode(&mut self) -> bool {
        let opcode = self.get_u8();
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
                let block_type = self.get_u8();
                println!("block {:x}", block_type);
                while self.peek_byte() != 0x0B {
                    self.match_opcode();
                }
                self.get_u8();
            }
            0x03 => {
                // loop
                let block_type = self.get_u8();
                println!("loop {:x}", block_type);
                while self.peek_byte() != 0x0B {
                    self.match_opcode();
                }
                self.get_u8();
            }
            0x04 => {
                // if
                let block_type = self.get_u8();
                // TODO: how to handle block_type??
                println!("if {:x}", block_type);
                while self.peek_byte() != 0x0B {
                    if self.peek_byte() == 0x05 {
                        self.get_u8();
                        while self.peek_byte() != 0x0B {
                            self.match_opcode();
                        }
                        self.get_u8();
                        return false;
                    }
                    self.match_opcode();
                }
                self.get_u8();
            }
            0x05 => {
                // else
                println!("else");
                // TODO: how to handle block_type??
                println!("start else");
                while self.peek_byte() != 0x0B {
                    self.match_opcode();
                }
                self.get_u8();
                println!("end else");
                // self.match_opcodes();
            }
            0x0B => {
                // function end byte
                println!("function end byte");
                return true;
            }
            0x0C => {
                // br
                let label_idx = self.get_u32();
                println!("br: {}", label_idx);
            }
            0x0D => {
                // br_if
                let label_idx = self.get_u32();
                println!("br_if: {}", label_idx);
            }
            0x0E => {
                // br_table
                print!("br_table: ");
                let label_size = self.get_u32();
                print!("label_size: {}", label_size);
                for _ in 0..label_size {
                    let label_idx = self.get_u32();
                    print!("\tlabel_idx: {}", label_idx);
                }
                let label_idx = self.get_u32();
                println!(" label_idx: {}", label_idx);
            }
            0x10 => {
                // call
                let func_idx = self.get_u32();
                println!("call {}", func_idx);
            }
            0x11 => {
                // call_indirect
                let type_idx = self.get_u32();
                println!("call_indirect {}", type_idx);
                let next_byte = self.get_u8();
                assert_eq!(next_byte, 0x00, "call_indirect not followed by 0x00");
            }
            0x20 => {
                // local.get
                let local_idx = self.get_u32();
                println!("local.get {}", local_idx);
            }
            0x21 => {
                // local.set
                let local_idx = self.get_u32();
                println!("local.set {}", local_idx);
            }
            0x2D => {
                // i32.load8_u
                let align = self.get_u32();
                let offset = self.get_u32();
                println!("i32.load8_u {} {}", align, offset);
            }
            0x36 => {
                // i32.store
                let align = self.get_u32();
                let offset = self.get_u32();
                println!("i32.store {} {}", align, offset);
            }
            0x41 => {
                // i32.const
                let val = self.get_u32();
                println!("i32.const: {}", val);
            }
            0x46 => {
                // i32.eq
                println!("i32.eq");
            }
            0x48 => {
                // i32.lt_s
                println!("i32.lt_s");
            }
            0x4E => {
                // i32.ge_s
                println!("i32.ge_s");
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
            0x72 => {
                //i32.or
                println!("i32.or");
            }
            0x74 => {
                //i32.shl
                println!("i32.shl");
            }
            0x76 => {
                //i32.shr_u
                println!("i32.shr_u");
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
        let val = self.get_u8();
        self.cursor.set_position(pos);
        val
    }

    fn get_u8(&mut self) -> u8 {
        let ret = self.cursor.read_u8();
        match ret {
            Ok(val) => val,
            Err(err) => {
                panic!(err);
            }
        }
    }

    fn get_u32(&mut self) -> u32 {
        let ret: Result<(u32, usize), wasabi_leb128::ParseLeb128Error> = self.cursor.read_leb128();
        match ret {
            Ok(val) => {
                let (val, _): (u32, _) = val;
                val
            }
            Err(err) => {
                // TODO: Clean this up later
                panic!(err);
            }
        }
    }

    fn get_leb128_value<T>(&mut self) -> T
    where
        T: PrimInt + 'static,
        u8: AsPrimitive<T>,
    {
        let (t, _): (T, _) = self.cursor.read_leb128().unwrap();
        t
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
