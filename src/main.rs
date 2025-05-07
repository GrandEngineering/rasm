mod lexer;
use std::io::Write;
fn main() {
    let insts = std::fs::read_to_string("ex.rasm").unwrap();
    let n = lexer::Program::new(insts);
    n.write_to_file("ex.bin").unwrap();
    println!("OK")
}
fn vm(insts: Vec<u32>) {
    let mut stack_mem: [u8; 2] = [0u8; 2];
    let mut memory: [u8; 256] = [0u8; 256];
    let mut program_mem: [u8; 1024] = [0u8; 1024]; // 256 lines of code
}
