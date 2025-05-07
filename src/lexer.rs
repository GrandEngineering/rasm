use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

pub enum Token {
    NOP,
    HALT,
    ADD,
    SUB,
    NOR,
    AND,
    XOR,
    RSH,
    LDI,
    ADI,
    JMP,
    BRH,
    CAL,
    RET,
    LOD,
    STR,
    BYTE(u8),
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Program {
    instructions: Vec<u32>,
}

impl Program {
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let mut file = File::create(path)?;
        let bytes = self.as_bytes();
        file.write_all(&bytes)?;
        Ok(())
    }
    pub fn new(instructions: String) -> Self {
        let mut program_instructions: Vec<u32> = Vec::new();

        for line in instructions.lines() {
            // Skip empty lines and comments
            let line = line.trim();
            if line.is_empty() || line.starts_with("#") {
                continue;
            }

            // Split by whitespace and handle trailing comments
            let parts: Vec<&str> = line
                .split('#')
                .next()
                .unwrap()
                .trim()
                .split_whitespace()
                .collect();
            if parts.is_empty() {
                continue;
            }

            let opcode = parts[0].to_uppercase();
            let mut instruction: u32 = 0;

            match opcode.as_str() {
                "NOP" => instruction = Self::encode_opcode(0x00),
                "HALT" => instruction = Self::encode_opcode(0x01),
                "ADD" => {
                    instruction = Self::encode_opcode(0x02);
                    if parts.len() >= 3 {
                        instruction |= Self::parse_register(parts[1]) << 8;
                        instruction |= Self::parse_register(parts[2]) << 16;
                    }
                }
                "SUB" => {
                    instruction = Self::encode_opcode(0x03);
                    if parts.len() >= 3 {
                        instruction |= Self::parse_register(parts[1]) << 8;
                        instruction |= Self::parse_register(parts[2]) << 16;
                    }
                }
                "NOR" => {
                    instruction = Self::encode_opcode(0x04);
                    if parts.len() >= 3 {
                        instruction |= Self::parse_register(parts[1]) << 8;
                        instruction |= Self::parse_register(parts[2]) << 16;
                    }
                }
                "AND" => {
                    instruction = Self::encode_opcode(0x05);
                    if parts.len() >= 3 {
                        instruction |= Self::parse_register(parts[1]) << 8;
                        instruction |= Self::parse_register(parts[2]) << 16;
                    }
                }
                "XOR" => {
                    instruction = Self::encode_opcode(0x06);
                    if parts.len() >= 3 {
                        instruction |= Self::parse_register(parts[1]) << 8;
                        instruction |= Self::parse_register(parts[2]) << 16;
                    }
                }
                "RSH" => {
                    instruction = Self::encode_opcode(0x07);
                    if parts.len() >= 2 {
                        instruction |= Self::parse_register(parts[1]) << 8;
                    }
                }
                "LDI" => {
                    instruction = Self::encode_opcode(0x08);
                    if parts.len() >= 3 {
                        instruction |= Self::parse_register(parts[1]) << 8;
                        instruction |= Self::parse_value(parts[2]) << 16;
                    }
                }
                "ADI" => {
                    instruction = Self::encode_opcode(0x09);
                    if parts.len() >= 3 {
                        instruction |= Self::parse_register(parts[1]) << 8;
                        instruction |= Self::parse_value(parts[2]) << 16;
                    }
                }
                "JMP" => {
                    instruction = Self::encode_opcode(0x0A);
                    if parts.len() >= 2 {
                        instruction |= Self::parse_value(parts[1]) << 8;
                    }
                }
                "BRH" => {
                    instruction = Self::encode_opcode(0x0B);
                    if parts.len() >= 3 {
                        instruction |= Self::parse_register(parts[1]) << 8;
                        instruction |= Self::parse_value(parts[2]) << 16;
                    }
                }
                "CAL" => {
                    instruction = Self::encode_opcode(0x0C);
                    if parts.len() >= 2 {
                        instruction |= Self::parse_value(parts[1]) << 8;
                    }
                }
                "RET" => instruction = Self::encode_opcode(0x0D),
                "LOD" => {
                    instruction = Self::encode_opcode(0x0E);
                    if parts.len() >= 3 {
                        // Check if we have the alternative syntax: LOD m0 R2
                        if parts[1].starts_with('m') || parts[1].starts_with('M') {
                            // Memory address first, register second
                            instruction |= Self::parse_register(parts[2]) << 8;
                            instruction |= Self::parse_memory_address(parts[1]) << 16;
                        } else {
                            // Traditional syntax: LOD R2 0x42
                            instruction |= Self::parse_register(parts[1]) << 8;
                            instruction |= Self::parse_value(parts[2]) << 16;
                        }
                    }
                }
                "STR" => {
                    instruction = Self::encode_opcode(0x0F);
                    if parts.len() >= 3 {
                        // Check if we have the alternative syntax: STR R2 m0
                        if parts[2].starts_with('m') || parts[2].starts_with('M') {
                            // Register first, memory address second
                            instruction |= Self::parse_register(parts[1]) << 8;
                            instruction |= Self::parse_memory_address(parts[2]) << 16;
                        } else {
                            // Traditional syntax: STR R2 0x42
                            instruction |= Self::parse_register(parts[1]) << 8;
                            instruction |= Self::parse_value(parts[2]) << 16;
                        }
                    }
                }
                "BYTE" => {
                    if parts.len() >= 2 {
                        instruction = Self::parse_value(parts[1]) & 0xFF;
                    }
                }
                _ => {
                    // Try to handle raw bytes (could be a label or error)
                    if let Ok(value) = Self::parse_value_result(opcode.as_str()) {
                        instruction = value & 0xFF;
                    } else {
                        println!("Warning: Unknown instruction: {}", opcode);
                    }
                }
            }

            program_instructions.push(instruction);
        }

        Program {
            instructions: program_instructions,
        }
    }
    fn parse_memory_address(mem_str: &str) -> u32 {
        let mem_str = mem_str.trim().to_uppercase();
        if mem_str.starts_with('M') {
            if let Ok(addr) = mem_str[1..].parse::<u32>() {
                return addr & 0xFF; // 8-bit address
            }
        }
        println!(
            "Warning: Invalid memory address: {}. Using address 0 instead.",
            mem_str
        );
        0
    }
    pub fn get_instructions(&self) -> &Vec<u32> {
        &self.instructions
    }

    // Encode an opcode into the instruction
    fn encode_opcode(opcode: u32) -> u32 {
        opcode & 0xFF
    }

    // Parse a register identifier (R0-R7)
    fn parse_register(reg_str: &str) -> u32 {
        let reg_str = reg_str.trim().to_uppercase();
        if reg_str.starts_with('R') {
            if let Ok(reg_num) = reg_str[1..].parse::<u32>() {
                if reg_num < 8 {
                    return reg_num & 0xFF;
                }
            }
        }
        println!("Warning: Invalid register: {}. Using R0 instead.", reg_str);
        0
    }

    // Parse a value in various formats (hex, binary, decimal)
    fn parse_value(val_str: &str) -> u32 {
        Self::parse_value_result(val_str).unwrap_or_else(|_| {
            println!("Warning: Invalid value: {}. Using 0 instead.", val_str);
            0
        })
    }

    // Helper method that returns a Result to handle errors
    fn parse_value_result(val_str: &str) -> Result<u32, &'static str> {
        let val_str = val_str.trim();

        // Hexadecimal (0x prefix or h suffix)
        if val_str.starts_with("0x") || val_str.starts_with("0X") {
            return u32::from_str_radix(&val_str[2..], 16).map_err(|_| "Invalid hex value");
        } else if val_str.ends_with('h') || val_str.ends_with('H') {
            return u32::from_str_radix(&val_str[..val_str.len() - 1], 16)
                .map_err(|_| "Invalid hex value");
        }
        // Binary (0b prefix or b suffix)
        else if val_str.starts_with("0b") || val_str.starts_with("0B") {
            return u32::from_str_radix(&val_str[2..], 2).map_err(|_| "Invalid binary value");
        } else if val_str.ends_with('b') || val_str.ends_with('B') {
            return u32::from_str_radix(&val_str[..val_str.len() - 1], 2)
                .map_err(|_| "Invalid binary value");
        }
        // Decimal (default)
        else {
            return val_str.parse::<u32>().map_err(|_| "Invalid decimal value");
        }
    }

    // Methods to work with the assembled program
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.instructions.len() * 4);
        for instruction in &self.instructions {
            bytes.push((instruction & 0xFF) as u8);
            bytes.push(((instruction >> 8) & 0xFF) as u8);
            bytes.push(((instruction >> 16) & 0xFF) as u8);
            bytes.push(((instruction >> 24) & 0xFF) as u8);
        }
        bytes
    }

    pub fn disassemble(&self) -> String {
        let mut output = String::new();
        for (i, instruction) in self.instructions.iter().enumerate() {
            let opcode = instruction & 0xFF;
            let reg1 = (instruction >> 8) & 0xFF;
            let reg2 = (instruction >> 16) & 0xFF;

            let formatted = match opcode {
                0x00 => "NOP".to_string(),
                0x01 => "HALT".to_string(),
                0x02 => format!("ADD R{}, R{}", reg1, reg2),
                0x03 => format!("SUB R{}, R{}", reg1, reg2),
                0x04 => format!("NOR R{}, R{}", reg1, reg2),
                0x05 => format!("AND R{}, R{}", reg1, reg2),
                0x06 => format!("XOR R{}, R{}", reg1, reg2),
                0x07 => format!("RSH R{}", reg1),
                0x08 => format!("LDI R{}, 0x{:02X}", reg1, reg2),
                0x09 => format!("ADI R{}, 0x{:02X}", reg1, reg2),
                0x0A => format!("JMP 0x{:02X}", reg1),
                0x0B => format!("BRH R{}, 0x{:02X}", reg1, reg2),
                0x0C => format!("CAL 0x{:02X}", reg1),
                0x0D => "RET".to_string(),
                0x0E => format!("LOD R{}, 0x{:02X}", reg1, reg2),
                0x0F => format!("STR R{}, 0x{:02X}", reg1, reg2),
                _ => format!("BYTE 0x{:02X}", opcode),
            };

            output.push_str(&format!("{:04X}: {}\n", i * 4, formatted));
        }
        output
    }
}
