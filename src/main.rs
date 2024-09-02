use std::any::Any;
use std::fs;
use std::io;
use std::u8;

use lexer::Token;
mod lexer;
fn main() -> io::Result<()> {
    let mut reader = fs::read_to_string("ex.rasm")?;
    let mut program_memory: [u8; 128] = [0; 128];
    let mut registers: [u8; 16] = [0; 16];
    let mut lexer = lexer::Lexer::new(reader);
    let mut argvbuf: [u8; 3] = [0; 3]; // argument buffer
    loop {
        let token = lexer.next_token();
        if token == lexer::Token::Eof {
            break;
        }
        println!("{:?}", token);
    }
    Ok(())
}
fn mem_to_access(m: &str) -> usize {
    m.strip_prefix("m")
        .and_then(|s| s.parse::<usize>().ok())
        .filter(|&n| n < 128)
        .unwrap_or_else(|| panic!("Invalid memory access: {}", m))
}
fn reg_to_access(u: &str) -> usize {
    u.strip_prefix("r")
        .and_then(|s| s.parse::<usize>().ok())
        .filter(|&n| n < 16)
        .unwrap_or_else(|| panic!("Invalid register access: {}", u))
}
fn nop() {}
fn add(r1: &str, r2: &str, r3: &str, mut r: [u8; 16]) -> [u8; 16] {
    let dr1 = reg_to_access(r1);
    let dr2 = reg_to_access(r2);
    let dr3 = reg_to_access(r3);
    r[dr3] = r[dr1] + r[dr2];
    r
}
fn move_reg(r1: &str, r2: &str, mut r: [u8; 16]) -> [u8; 16] {
    let dr1 = reg_to_access(r1);
    let dr2 = reg_to_access(r2);
    r[dr2] = r[dr1];
    r
}
fn sub(r1: &str, r2: &str, r3: &str, mut r: [u8; 16]) -> [u8; 16] {
    let dr1 = reg_to_access(r1);
    let dr2 = reg_to_access(r2);
    let dr3 = reg_to_access(r3);
    r[dr3] = r[dr1] - r[dr2];
    r
}
fn nor(r1: &str, r2: &str, r3: &str, mut r: [u8; 16]) -> [u8; 16] {
    let dr1 = reg_to_access(r1);
    let dr2 = reg_to_access(r2);
    let dr3 = reg_to_access(r3);
    r[dr3] = !(r[dr1] | r[dr2]);
    r
}
fn and(r1: &str, r2: &str, r3: &str, mut r: [u8; 16]) -> [u8; 16] {
    let dr1 = reg_to_access(r1);
    let dr2 = reg_to_access(r2);
    let dr3 = reg_to_access(r3);
    r[dr3] = r[dr1] & r[dr2];
    r
}
fn xor(r1: &str, r2: &str, r3: &str, mut r: [u8; 16]) -> [u8; 16] {
    let dr1 = reg_to_access(r1);
    let dr2 = reg_to_access(r2);
    let dr3 = reg_to_access(r3);
    r[dr3] = r[dr1] ^ r[dr2];
    r
}
fn rsh(r1: &str, r2: &str, r3: &str, mut r: [u8; 16]) -> [u8; 16] {
    let dr1 = reg_to_access(r1);
    let dr2 = reg_to_access(r2);
    let dr3 = reg_to_access(r3);
    r[dr3] = r[dr1] >> r[dr2];
    r
}
fn lsh(r1: &str, r2: &str, r3: &str, mut r: [u8; 16]) -> [u8; 16] {
    let dr1 = reg_to_access(r1);
    let dr2 = reg_to_access(r2);
    let dr3 = reg_to_access(r3);
    r[dr3] = r[dr1] << r[dr2];
    r
}
fn load(m0: &str, r1: &str, mut reg: [u8; 16], mut mem: [u8; 128]) -> ([u8; 16], [u8; 128]) {
    let dr1 = reg_to_access(r1);
    let dm0 = mem_to_access(m0);
    reg[dr1] = mem[dm0];
    (reg, mem)
}
fn store(m0: &str, r1: &str, mut reg: [u8; 16], mut mem: [u8; 128]) -> ([u8; 16], [u8; 128]) {
    let dr1 = reg_to_access(r1);
    let dm0 = mem_to_access(m0);
    mem[dm0] = reg[dr1];
    (reg, mem)
}
fn jmp() {}
fn match_jmp() {}
