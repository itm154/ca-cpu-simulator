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
    pub program: Vec<u16>,
    pub memory_list_state: ListState,
    pub register_logs: Vec<String>,
    pub register_logs_list_state: ListState,
    pub step_mode: bool,
    pub exit: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            cpu: CPU::default(),
            program: Vec::default(),
            memory_list_state: ListState::default(),
            register_logs: Vec::default(),
            register_logs_list_state: ListState::default(),
            exit: false,
            step_mode: true, // Start in step mode by default
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

            // NOTE: Operation type
            // Non step mode operation
            // Step mode is defined in handle_key_event
            if !self.cpu.halted && !self.step_mode {
                self.cpu.fetch();
                let (opcode, register, operand) = self.cpu.decode();
                self.cpu.execute(opcode, register, operand);
                self.register_logs.push(self.cpu.log_registers());
                self.memory_list_state
                    .select(Some(self.cpu.pc.saturating_sub(1) as usize)); // Highlight current
            }
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        // Define layout in terminal (3 Horizontally split panes)
        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Length(33),
                Constraint::Fill(1),
                Constraint::Length(34),
            ])
            .split(frame.area());

        // == Memory List widget ==
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
        // =+= Memory List widget =+=

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

        // == Register Logs widget ==
        let register_logs_item: Vec<ListItem> = self
            .register_logs
            .iter()
            .enumerate()
            .map(|(_, log)| ListItem::new(log.clone()))
            .collect();

        let register_logs_widget = List::new(register_logs_item)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Register Logs"),
            )
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

        frame.render_stateful_widget(
            register_logs_widget,
            main_layout[2],
            &mut self.register_logs_list_state,
        );
        // =+= Register Logs widget =+=
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
            KeyCode::Enter => self.step(),
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('r') => self.reset_cpu(),
            KeyCode::Up => self.scroll_memory_up(),
            KeyCode::Down => self.scroll_memory_down(),
            KeyCode::Char('t') => {
                self.step_mode = !self.step_mode; // Toggle
            }
            _ => {}
        }
    }

    fn step(&mut self) {
        if self.step_mode && !self.cpu.halted {
            self.cpu.fetch();
            let (opcode, register, operand) = self.cpu.decode();
            self.cpu.execute(opcode, register, operand);
            self.register_logs.push(self.cpu.log_registers());
        }
        self.memory_list_state
            .select(Some(self.cpu.pc.saturating_sub(1) as usize)); // Highlight current
        // instruction in memory
    }

    fn reset_cpu(&mut self) {
        self.cpu.halted = false;
        self.cpu.pc = 0;
        self.register_logs.clear();
        self.cpu.memory = [0u16; 64];
        self.memory_list_state = ListState::default();
        self.cpu = CPU::new(&self.program);
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
