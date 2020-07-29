use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;

mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let buffer = get_file_as_byte_vec(filename);
    let mut p = parser::Parser::new(&buffer);
    p.parse();
}

fn get_file_as_byte_vec(filename: &str) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read_exact(&mut buffer).expect("buffer overflow");

    buffer
}
