mod app;
mod cpu;

use crate::app::App;
use crate::cpu::CPU;

use std::{fs, io};

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
