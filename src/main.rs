pub struct CPU {
    r0: u16,           // General purpose register
    r1: u16,           // following RISC naming convention (R[n])
    r2: u16,           // probably dont need this many
    r3: u16,           // registers but why not
    pc: u16,           // Program counter
    ir: u16,           // Instruction register
    memory: [u16; 64], // 64 bytes of memory
    halted: bool,      // For HALT opcode
}

// NOTE: Placeholder opcodes for now
enum OpCode {
    LOAD = 0b0001,
    STORE = 0b0010,
    ADD = 0b0011,
    SUB = 0b0100,
    JMP = 0b0101,
    HALT = 0b0000,
}

impl OpCode {
    // Helper function for execute cycle
    pub fn u8_to_opcode(value: u8) -> Option<OpCode> {
        match value {
            0b0000 => Some(OpCode::HALT),
            0b0001 => Some(OpCode::LOAD),
            0b0010 => Some(OpCode::STORE),
            0b0011 => Some(OpCode::ADD),
            0b0100 => Some(OpCode::SUB),
            0b0101 => Some(OpCode::JMP),
            _ => None,
        }
    }
}

impl CPU {
    // Initialize a new CPU with program
    pub fn new(program: &[u16]) -> CPU {
        // Initialize memory with 0s and copy program to memory
        let mut memory = [0u16; 64];
        memory[..program.len()].copy_from_slice(program);

        CPU {
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            pc: 0,
            ir: 0,
            memory,
            halted: false,
        }
    }

    pub fn fetch(&mut self) {
        // Read at instruction at program counter
        self.ir = self.memory[self.pc as usize];
        self.pc = self.pc.wrapping_add(1); // Fancy +1/increment
    }

    pub fn decode(&self) -> (u8, u8, u8) {
        let instruction = self.ir;

        // black magic to extract bits from IR
        // >> is bit shift
        // & is bit mask (same princible as subnet mask in CCN)
        let opcode = ((instruction >> 12) & 0b1111) as u8;
        let register = ((instruction >> 8) & 0b1111) as u8;
        let operand = (instruction & 0b1111_1111) as u8;

        (opcode, register, operand)
    }

    pub fn execute(&mut self, opcode: u8, register: u8, operand: u8) {
        use OpCode::*;

        match OpCode::u8_to_opcode(opcode) {
            // SOME and NONE
            // NOTE: https://stackoverflow.com/questions/24771655/what-are-some-and-none
            Some(HALT) => {
                self.halted = true;
            }
            Some(LOAD) => {}

            Some(STORE) => {}

            Some(ADD) => {}

            Some(SUB) => {}

            Some(JMP) => {}
            None => {
                panic!("Unknown opcode {:#04b}", opcode)
            }
        }
    }

    pub fn print() {}
}

fn main() {
    let program = [];

    let mut cpu = CPU::new(&program);

    while !cpu.halted {
        cpu.fetch();
        let (opcode, register, operand) = cpu.decode();
        cpu.execute(opcode, register, operand);

        // Print values of the registers after each cycle
        // cpu.print();
    }

    println!("CPU halted");
}
