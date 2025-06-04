use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, ListItem, Paragraph, Widget},
};

use std::{default, fs, io};

#[derive(Debug)]
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

impl Default for CPU {
    fn default() -> Self {
        CPU {
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

#[derive(Debug, Clone, Copy)]
enum OpCode {
    HALT = 0b0000,
    LVAL = 0b0001, // Load immediate
    LOAD = 0b0010, // Load from memory
    STORE = 0b011, // Store value of register to memory
    ADD = 0b0100,
    SUB = 0b0101,
    JMP = 0b0110,
    MOV = 0b0111, // Move register's value to another register
}

impl OpCode {
    // Helper function for execute cycle
    pub fn u8_to_opcode(value: u8) -> Option<OpCode> {
        match value {
            0b0000 => Some(OpCode::HALT),
            0b0001 => Some(OpCode::LVAL),
            0b0011 => Some(OpCode::LOAD),
            0b0100 => Some(OpCode::STORE),
            0b0101 => Some(OpCode::ADD),
            0b0110 => Some(OpCode::SUB),
            0b0111 => Some(OpCode::JMP),
            0b1000 => Some(OpCode::MOV),
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

    fn get_register_mut(&mut self, index: u8) -> &mut u16 {
        match index & 0b11 {
            0b00 => &mut self.r0,
            0b01 => &mut self.r1,
            0b10 => &mut self.r2,
            0b11 => &mut self.r3,
            _ => unreachable!(),
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

    pub fn fetch(&mut self) {
        // Read at instruction at program counter
        self.ir = self.memory[self.pc as usize];
        self.pc = self.pc.wrapping_add(1); // Fancy +1/increment
    }

    pub fn decode(&self) -> (u8, u8, u8) {
        let instruction = self.ir;

        // black magic to extract bits from IR
        // >> is bitwise shift
        // & is bitwise AND (same princible as subnet mask in CCN)
        // NOTE: https://www.tutorialspoint.com/rust/rust_bitwise_operators.htm
        let opcode = ((instruction >> 12) & 0b1111) as u8;
        let register = ((instruction >> 8) & 0b1111) as u8;
        let operand = (instruction & 0b1111_1111) as u8;

        (opcode, register, operand)
    }

    pub fn execute(&mut self, opcode: u8, register: u8, operand: u8) {
        use OpCode::*;

        match OpCode::u8_to_opcode(opcode) {
            // NOTE: https://stackoverflow.com/questions/24771655/what-are-some-and-none
            Some(HALT) => {
                self.halted = true;
            }

            Some(LVAL) => {
                *self.get_register_mut(register) = operand as u16;
            }

            Some(LOAD) => {
                *self.get_register_mut(register) = self.memory[operand as usize];
            }

            Some(STORE) => {
                let value = self.get_register(register);
                //
                // Store current value in register into memory
                self.memory[operand as usize] = value;
            }

            Some(ADD) => {
                let dest = self.get_register(register);
                let src = self.get_register(operand);
                let dest_reg_mut = self.get_register_mut(register);
                *dest_reg_mut = dest.wrapping_add(src);
            }

            Some(SUB) => {
                let dest = self.get_register(register);
                let src = self.get_register(operand);
                let dest_reg_mut = self.get_register_mut(register);
                *dest_reg_mut = dest.wrapping_sub(src);
            }

            Some(JMP) => {
                self.pc = operand as u16;
            }

            Some(MOV) => {
                let src = self.get_register(operand);
                let dest = self.get_register_mut(register);
                *dest = src;
            }

            None => {
                panic!("Unknown opcode {:#04b}", opcode)
            }
        }
    }
}

// TUI Implementation
#[derive(Default)]
pub struct App {
    cpu: CPU,
    logs: Vec<String>,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(area);

        let memory_layout = layout[0];
        let log_layout = layout[1];

        // let memory_item: Vec<ListItem> =
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let program_bin = fs::read("program.bin").unwrap_or_default();

    let program: Vec<u16> = program_bin
        .chunks_exact(2)
        .map(|chunk| u16::from_le_bytes([chunk[1], chunk[0]]))
        .collect();

    let mut app = App {
        cpu: CPU::new(&program),
        ..Default::default()
    };

    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}
