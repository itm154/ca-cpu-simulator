use crate::cpu::CPU;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use std::io;

pub struct App {
    pub cpu: CPU,
    pub memory_list_state: ListState,
    pub exit: bool,
    pub step_mode: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            cpu: CPU::default(),
            memory_list_state: ListState::default(),
            exit: false,
            step_mode: true, // Start in step mode by default, or false for auto-run
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.memory_list_state.select(Some(0));

        loop {
            terminal.draw(|frame| self.draw(frame))?;

            self.handle_events()?;

            if self.exit {
                break;
            }

            // NOTE:
            // Non step mode operation
            // Step mode is defined in handle_key_event
            if !self.cpu.halted && !self.step_mode {
                self.cpu.fetch();
                let (opcode, register, operand) = self.cpu.decode();
                self.cpu.execute(opcode, register, operand);
            }
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Length(33), Constraint::Fill(1)])
            .split(frame.area());

        let memory_items: Vec<ListItem> = self
            .cpu
            .memory
            .iter()
            .enumerate()
            .map(|(i, &val)| ListItem::new(format!(" {:2} │ {:016b} │ {:5}", i, val, val)))
            .collect();

        let memory_list_widget = List::new(memory_items)
            .block(Block::default().borders(Borders::ALL).title("Memory view"))
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

        frame.render_stateful_widget(
            memory_list_widget,
            main_layout[0],
            &mut self.memory_list_state,
        );

        // == CPU status widget ==
        let registers = self.cpu.get_all_registers();
        let cpu_status_text = format!(
            "PC: {} \nIR: {:016b}\n\nR0: {}\nR1: {}\nR2: {}\nR3: {} \n\nHalted: {}",
            self.cpu.pc,
            self.cpu.ir,
            registers[0],
            registers[1],
            registers[2],
            registers[3],
            self.cpu.halted
        );

        let cpu_status_paragraph = Paragraph::new(cpu_status_text)
            .block(Block::default().borders(Borders::ALL).title("CPU Status"))
            .wrap(ratatui::widgets::Wrap { trim: false });

        frame.render_widget(cpu_status_paragraph, main_layout[1]);
        // =+= CPU status widget =+=
    }

    fn handle_events(&mut self) -> io::Result<()> {
        // Poll to prevent blocking
        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Press {
                    self.handle_key_event(key_event);
                }
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Up => self.scroll_memory_up(),
            KeyCode::Down => self.scroll_memory_down(),
            KeyCode::Enter => {
                // NOTE: CPU Cycle
                if self.step_mode && !self.cpu.halted {
                    self.cpu.fetch();
                    let (opcode, register, operand) = self.cpu.decode();
                    self.cpu.execute(opcode, register, operand);
                }
                self.memory_list_state
                    .select(Some(self.cpu.pc.saturating_sub(1) as usize)); // Highlight current
                // instruction in memory
            }
            KeyCode::Char('t') => {
                self.step_mode = !self.step_mode;
            }
            _ => {}
        }
    }

    fn scroll_memory_up(&mut self) {
        let i = match self.memory_list_state.selected() {
            Some(selected) => {
                if selected == 0 {
                    self.cpu.memory.len() - 1 // wrap to the end
                } else {
                    selected - 1
                }
            }
            None => 0,
        };
        self.memory_list_state.select(Some(i));
    }

    fn scroll_memory_down(&mut self) {
        let i = match self.memory_list_state.selected() {
            Some(selected) => {
                if selected >= self.cpu.memory.len() - 1 {
                    0 // wrap to beginning
                } else {
                    selected + 1
                }
            }
            None => 0,
        };
        self.memory_list_state.select(Some(i));
    }
}
