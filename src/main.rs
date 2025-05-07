mod lexer;
use std::io::Write;
fn main() {
    let insts = std::fs::read_to_string("ex.rasm").unwrap();
    let n = lexer::Program::new(insts);
    n.write_to_file("ex.bin").unwrap();

    vm(n.get_instructions());
}
fn vm(insts: &Vec<u32>) {
    let mut stack_mem: [u8; 2] = [0u8; 2];
    let mut memory: [u8; 256] = [0u8; 256]; // Used like r0 to r255
    let mut store: [u8; 1024] = [0u8; 1024];
    let mut program_mem: [u8; 2048] = [0u8; 2048]; // 512 lines of code

    const PRINT_CHAR_ADDR: usize = 2;

    for (i, inst) in insts.iter().enumerate() {
        if i * 4 + 3 < program_mem.len() {
            program_mem[i * 4] = (inst & 0xFF) as u8;
            program_mem[i * 4 + 1] = ((inst >> 8) & 0xFF) as u8;
            program_mem[i * 4 + 2] = ((inst >> 16) & 0xFF) as u8;
            program_mem[i * 4 + 3] = ((inst >> 24) & 0xFF) as u8;
        } else {
            println!("Warning: Program too large for program memory");
            break;
        }
    }

    let mut pc: usize = 0; // Program counter
    let mut sp: usize = 0; // Stack pointer
    let mut running = true;

    while running && pc < program_mem.len() / 4 {
        // Fetch instruction
        let opcode = program_mem[pc * 4] as u32;
        let reg1 = program_mem[pc * 4 + 1] as u32;
        let reg2 = program_mem[pc * 4 + 2] as u32;
        let reg3 = program_mem[pc * 4 + 3] as u32;

        // Increment PC before execution
        pc += 1;

        // Execute instruction
        match opcode {
            0x00 => {
                // NOP
            }
            0x01 => {
                // HALT
                running = false;
            }
            0x02 => {
                // ADD
                memory[reg1 as usize] = memory[reg1 as usize].wrapping_add(memory[reg2 as usize]);
            }
            0x03 => {
                // SUB
                memory[reg1 as usize] = memory[reg1 as usize].wrapping_sub(memory[reg2 as usize]);
            }
            0x04 => {
                // NOR
                memory[reg1 as usize] = !(memory[reg1 as usize] | memory[reg2 as usize]);
            }
            0x05 => {
                // AND
                memory[reg1 as usize] &= memory[reg2 as usize];
            }
            0x06 => {
                // XOR
                memory[reg1 as usize] ^= memory[reg2 as usize];
            }
            0x07 => {
                // RSH - Right shift
                memory[reg1 as usize] >>= 1;
            }
            0x08 => {
                // LDI - Load immediate
                memory[reg1 as usize] = reg2 as u8;
            }
            0x09 => {
                // ADI - Add immediate
                memory[reg1 as usize] = memory[reg1 as usize].wrapping_add(reg2 as u8);
            }
            0x0A => {
                // JMP - Jump to address
                pc = reg1 as usize;
            }
            0x0B => {
                // BRH - Branch if not zero
                if memory[reg1 as usize] != 0 {
                    pc = reg2 as usize;
                }
            }
            0x0C => {
                // CAL - Call subroutine
                if sp < stack_mem.len() {
                    stack_mem[sp] = pc as u8;
                    sp += 1;
                    pc = reg1 as usize;
                } else {
                    println!("Stack overflow on CAL instruction");
                    running = false;
                }
            }
            0x0D => {
                // RET - Return from subroutine
                if sp > 0 {
                    sp -= 1;
                    pc = stack_mem[sp] as usize;
                } else {
                    println!("Stack underflow on RET instruction");
                    running = false;
                }
            }
            0x0E => {
                // LOD - Load from memory
                if (reg2 as usize) < store.len() {
                    memory[reg1 as usize] = store[reg2 as usize];
                } else {
                    println!("Memory access violation on LOD instruction");
                    running = false;
                }
            }
            0x0F => {
                // STR - Store to memory
                if (reg2 as usize) < store.len() {
                    store[reg2 as usize] = memory[reg1 as usize];

                    // Special handling for printing ASCII if writing to PRINT_CHAR_ADDR (m2)
                    if reg2 as usize == PRINT_CHAR_ADDR {
                        let char_to_print = memory[reg1 as usize];
                        print!("{}", char_to_print as char);
                        // Flush to make sure character is displayed immediately
                        std::io::stdout().flush().unwrap();
                    }
                } else {
                    println!("Memory access violation on STR instruction");
                    running = false;
                }
            }
            _ => {
                println!("Unknown opcode: 0x{:02X}", opcode);
                running = false;
            }
        }

        // Debug output
        if !running {
            println!("Program halted at PC={:04X}", (pc - 1) * 4);
        }
    }

    // Add a newline after all output
    println!();

    // Print the final state of registers
    // println!("\nNon-zero Registers:");
    // for i in 0..256 {
    //     if memory[i] != 0 {
    //         println!("R{}: 0x{:02X} ({})", i, memory[i], memory[i]);
    //     }
    // }
}
