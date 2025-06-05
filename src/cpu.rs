#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    HALT = 0b0000,
    LVAL = 0b0001,
    LOAD = 0b0010,
    STORE = 0b0011,
    ADD = 0b0100,
    SUB = 0b0101,
    JMP = 0b0110,
    MOV = 0b0111,
}

impl OpCode {
    pub fn u8_to_opcode(value: u8) -> Option<OpCode> {
        match value {
            0b0000 => Some(OpCode::HALT),
            0b0001 => Some(OpCode::LVAL),
            0b0010 => Some(OpCode::LOAD),
            0b0011 => Some(OpCode::STORE),
            0b0100 => Some(OpCode::ADD),
            0b0101 => Some(OpCode::SUB),
            0b0110 => Some(OpCode::JMP),
            0b0111 => Some(OpCode::MOV),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct CPU {
    r0: u16,
    r1: u16,
    r2: u16,
    r3: u16,
    pc: u16,
    ir: u16,
    pub memory: [u16; 64],
    pub halted: bool,
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            pc: 0,
            ir: 0,
            memory: [0; 64],
            halted: false,
        }
    }
}

impl CPU {
    pub fn new(program: &[u16]) -> Self {
        let mut memory = [0u16; 64];
        memory[..program.len()].copy_from_slice(program);
        Self {
            memory,
            ..Default::default()
        }
    }

    pub fn fetch(&mut self) {
        self.ir = self.memory[self.pc as usize];
        self.pc = self.pc.wrapping_add(1);
    }

    pub fn decode(&self) -> (u8, u8, u8) {
        let opcode = ((self.ir >> 12) & 0b1111) as u8;
        let register = ((self.ir >> 8) & 0b1111) as u8;
        let operand = (self.ir & 0xFF) as u8;
        (opcode, register, operand)
    }

    pub fn execute(&mut self, opcode: u8, register: u8, operand: u8) {
        use OpCode::*;
        match OpCode::u8_to_opcode(opcode) {
            Some(HALT) => self.halted = true,
            Some(LVAL) => *self.get_register_mut(register) = operand as u16,
            Some(LOAD) => *self.get_register_mut(register) = self.memory[operand as usize],
            Some(STORE) => self.memory[operand as usize] = self.get_register(register),
            Some(ADD) => {
                let result = self
                    .get_register(register)
                    .wrapping_add(self.get_register(operand));
                *self.get_register_mut(register) = result;
            }
            Some(SUB) => {
                let result = self
                    .get_register(register)
                    .wrapping_sub(self.get_register(operand));
                *self.get_register_mut(register) = result;
            }
            Some(JMP) => self.pc = operand as u16,
            Some(MOV) => *self.get_register_mut(register) = self.get_register(operand),
            None => panic!("Unknown opcode: {:#04b}", opcode),
        }
    }

    fn get_register(&self, index: u8) -> u16 {
        match index & 0b11 {
            0b00 => self.r0,
            0b01 => self.r1,
            0b10 => self.r2,
            0b11 => self.r3,
            _ => unreachable!(),
        }
    }

    fn get_register_mut(&mut self, index: u8) -> &mut u16 {
        match index & 0b11 {
            0b00 => &mut self.r0,
            0b01 => &mut self.r1,
            0b10 => &mut self.r2,
            0b11 => &mut self.r3,
            _ => unreachable!(),
        }
    }
}
