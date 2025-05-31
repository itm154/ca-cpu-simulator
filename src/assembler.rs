use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

enum OpCode {
    HALT = 0b0000,
    LVAL = 0b0001,  // Load immediate value into register
    LOAD = 0b0011,  // Load value from memory into register
    STORE = 0b0100, // Store value of register to memory
    ADD = 0b0101,
    SUB = 0b0110,
    JMP = 0b0111,
    MOV = 0b1000, // Move register's value to another register
}

enum Register {
    R0 = 0b0000,
    R1 = 0b0001,
    R2 = 0b0010,
    R3 = 0b0011,
}

fn main() {
    let mut args = std::env::args();
    let filename = args.nth(1).unwrap();

    match std::fs::File::open(filename) {
        Ok(mut file) => {
            let mut output_file = File::create("program.bin").unwrap();

            let mut source = String::new();
            file.read_to_string(&mut source).unwrap();

            let lines: Vec<&str> = source.lines().collect();

            for line in lines {
                let first_char = line.chars().next().unwrap_or(';');

                // Ignore lines that starts with ; (comments)
                if first_char == ';' || first_char == '\n' {
                    continue;
                }

                let (instruction, _) = line.split_once(char::is_whitespace).unwrap();

                match instruction {
                    "HALT" => {
                        let binary_instruction = (OpCode::HALT as u16) << 12;

                        output_file
                            .write(binary_instruction.to_be_bytes().as_slice())
                            .unwrap();
                    }
                    "LVAL" => {
                        let (_, args) = line.split_once(char::is_whitespace).unwrap();
                        let (arg1, arg2) = args.split_once(", ").unwrap();
                        let mut binary_instructions = (OpCode::LVAL as u16) << 12;

                        match arg1 {
                            "R0" => {
                                binary_instructions =
                                    binary_instructions | (Register::R0 as u16) << 8;
                            }
                            "R1" => {
                                binary_instructions =
                                    binary_instructions | (Register::R1 as u16) << 8;
                            }
                            "R2" => {
                                binary_instructions =
                                    binary_instructions | (Register::R2 as u16) << 8;
                            }
                            "R3" => {
                                binary_instructions =
                                    binary_instructions | (Register::R3 as u16) << 8;
                            }
                            _ => {
                                panic!("Unknown register {arg1}")
                            }
                        }

                        binary_instructions =
                            binary_instructions | arg2.to_string().parse::<u16>().unwrap();

                        output_file
                            .write(binary_instructions.to_be_bytes().as_slice())
                            .unwrap();
                    }
                    "LOAD" => {
                        let (_, args) = line.split_once(char::is_whitespace).unwrap();
                        let (arg1, arg2) = args.split_once(", ").unwrap();
                        let mut binary_instructions = (OpCode::LVAL as u16) << 12;

                        match arg1 {
                            "R0" => {
                                binary_instructions =
                                    binary_instructions | (Register::R0 as u16) << 8;
                            }
                            "R1" => {
                                binary_instructions =
                                    binary_instructions | (Register::R1 as u16) << 8;
                            }
                            "R2" => {
                                binary_instructions =
                                    binary_instructions | (Register::R2 as u16) << 8;
                            }
                            "R3" => {
                                binary_instructions =
                                    binary_instructions | (Register::R3 as u16) << 8;
                            }
                            _ => {
                                panic!("Unknown register {arg1}")
                            }
                        }

                        binary_instructions =
                            binary_instructions | arg2.to_string().parse::<u16>().unwrap();

                        output_file
                            .write(binary_instructions.to_be_bytes().as_slice())
                            .unwrap();
                    }
                    "STORE" => {}
                    "ADD" => {}
                    "SUB" => {}
                    "JMP" => {}
                    "MOV" => {}
                    _ => {
                        println!("Unknown instruction {}", line)
                    }
                }
            }
        }
        Err(e) => {
            panic!("{e}");
        }
    }
}
