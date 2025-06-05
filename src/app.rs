use crate::cpu::CPU;

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind}
};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Widget},
};

use std::io;

pub struct App {
    pub cpu: CPU,
    pub logs: Vec<String>,
    pub memory_list_state: ListState,
    pub instruction_list_state: ListState,
    pub exit: bool,
    pub step_mode: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            cpu: CPU::default(), // Assuming CPU has a default implementation
            logs: Vec::new(),
            memory_list_state: ListState::default(),
            instruction_list_state: ListState::default(),
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
            .constraints(vec![Constraint::Length(35), Constraint::Fill(1)])
            .split(frame.area());

        let memory_items: Vec<ListItem> = self
            .cpu
            .memory
            .iter()
            .enumerate()
            .map(|(i, &val)| ListItem::new(format!(" {:2} │ {:018b} │ {:5}", i, val, val)))
            .collect();

        let memory_list_widget = List::new(memory_items)
            .block(Block::default().borders(Borders::ALL).title("Memory view"))
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

        frame.render_stateful_widget(
            memory_list_widget,
            main_layout[0],
            &mut self.memory_list_state,
        );

        let cpu_pane_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_layout[1]);

        let instruction_items:  Vec<ListItem> = self;

        let instruction_list_widget = List::new(instruction_items)
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                self.handle_key_event(key_event);
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
                // <--- New: Handle Enter key
                if self.step_mode && !self.cpu.halted {
                    // Perform one CPU cycle
                    self.cpu.fetch();
                    let (opcode, register, operand) = self.cpu.decode();
                    self.cpu.execute(opcode, register, operand);
                }
            }
            KeyCode::Char('t') => {
                // Optional: Toggle step mode vs auto-run
                self.step_mode = !self.step_mode;
                // Optionally log mode change
                self.logs.push(format!(
                    "Toggled step mode: {}",
                    if self.step_mode { "ON" } else { "OFF" }
                ));
            }
            _ => {}
        }
    }

    fn scroll_memory_up(&mut self) {
        let i = match self.memory_list_state.selected() {
            Some(selected) => {
                if selected == 0 {
                    self.cpu.memory.len() - 1 // Wrap around to the end
                } else {
                    selected - 1
                }
            }
            None => 0, // If nothing is selected, select the first item
        };
        self.memory_list_state.select(Some(i));
    }

    fn scroll_memory_down(&mut self) {
        let i = match self.memory_list_state.selected() {
            Some(selected) => {
                if selected >= self.cpu.memory.len() - 1 {
                    0 // Wrap around to the beginning
                } else {
                    selected + 1
                }
            }
            None => 0, // If nothing is selected, select the first item
        };
        self.memory_list_state.select(Some(i));
    }
}
