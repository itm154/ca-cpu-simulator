use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

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
                if first_char == ';' || first_char == '\n' {
                    continue;
                }

                let (instruction, _) = line.split_once(char::is_whitespace).unwrap();

                match instruction {
                    "HALT" => {}
                    "LVAL" => {}
                    "LOAD" => {}
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
