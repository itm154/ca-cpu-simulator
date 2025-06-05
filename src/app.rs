use crate::cpu::CPU;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Widget},
};

use std::io;

#[derive(Default)]
pub struct App {
    pub cpu: CPU,
    pub logs: Vec<String>,
    pub memory_list_state: ListState,
    pub exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let outer_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(frame.area());

        let inner_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(80), Constraint::Percentage(20)])
            .split(outer_layout[0]);

        let memory_items: Vec<ListItem> = self
            .cpu
            .memory
            .iter()
            .enumerate()
            .map(|(i, &val)| ListItem::new(format!("{:2}: {:018b} ({:5})", i, val, val)))
            .collect();

        let memory_list_widget = List::new(memory_items)
            .block(Block::default().borders(Borders::ALL).title("Memory view"))
            .highlight_symbol("> ");

        frame.render_stateful_widget(
            memory_list_widget,
            inner_layout[0],
            &mut self.memory_list_state,
        );
        frame.render_widget(
            Block::new().borders(Borders::ALL).title("Registers"),
            inner_layout[1],
        );
        frame.render_widget(
            Block::new().borders(Borders::ALL).title("CPU"),
            outer_layout[1],
        );
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
            _ => {}
        }
    }
}

impl Widget for &App {
    fn render(self, _area: Rect, _buf: &mut Buffer) {}
}
