use crossterm::{
    cursor::{Hide, MoveTo},
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Write};

pub struct Rect {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

pub fn draw_module(rect: Rect) {
    move_cursor(rect.x, rect.y);
    for _ in 1..rect.width {
        print!("-");
    }
    for y in 0..rect.height {
        move_cursor(rect.x, rect.y + y + 1);
        print!("|");
    }
    move_cursor(rect.x, rect.y + rect.height);
    for _ in 1..rect.width {
        print!("-");
    }
    for y in 1..rect.height {
        move_cursor(rect.x + rect.width - 1, rect.y + y);
        print!("|");
    }
}

pub fn draw_char(c: char, x: i32, y: i32) {
    move_cursor(x, y);
    print!("{c}");
    io::stdout().flush().unwrap();
}

pub fn move_cursor(x: i32, y: i32) {
    crossterm::execute!(std::io::stdout(), MoveTo(x as u16, y as u16)).unwrap()
}

pub fn clear() {
    crossterm::execute!(std::io::stdout(), Clear(ClearType::All)).unwrap()
}

pub fn leave_screen() {
    crossterm::execute!(std::io::stdout(), LeaveAlternateScreen).ok();
    crossterm::terminal::disable_raw_mode().ok();
}

pub fn enter_screen() {
    crossterm::execute!(std::io::stdout(), EnterAlternateScreen).ok();
    crossterm::terminal::enable_raw_mode().ok();
}

pub fn hide_cursor() {
    crossterm::execute!(std::io::stdout(), Hide).unwrap();
}
