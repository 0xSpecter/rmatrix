use crossterm::event::{Event, KeyCode};
use crossterm::style::{
    Attribute, Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor,
};
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{QueueableCommand, cursor, event, execute, terminal};
use rand::random_range;
use std::io::{Stdout, Write, stdout};

static CHARSET: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b',
    'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u',
    'v', 'w', 'x', 'y', 'z', '@', '#', '$', '%', '&', '*', '+', '-', '=', '?', '!', '<', '>',
];

pub struct Matrix {
    out: Stdout,
    cols: u16,
    rows: u16,
    buffer: Vec<Vec<char>>,
    background: Color,
    foreground: Color,
}

impl Matrix {
    pub fn new() -> Self {
        let (cols, rows) = terminal::size().unwrap();
        terminal::enable_raw_mode().unwrap();
        execute!(stdout(), EnterAlternateScreen, cursor::Hide).unwrap();

        Self {
            out: stdout(),
            buffer: vec![vec![' '; rows as usize]; cols as usize],
            cols,
            rows,
            background: Color::Black,
            foreground: Color::Green,
        }
    }

    fn cleanup(&mut self) {
        execute!(stdout(), LeaveAlternateScreen).unwrap();
        terminal::disable_raw_mode().unwrap();
    }

    fn clear(&mut self) {
        self.buffer = vec![vec![' '; self.rows as usize]; self.cols as usize];
    }

    fn handle_events(&mut self) {
        if event::poll(std::time::Duration::from_millis(0)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                // exit
                if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                    self.cleanup();
                    std::process::exit(0);
                }
            }
        }
    }

    fn handle_resize(&mut self) {
        let (ncols, nrows) = terminal::size().unwrap();
        let change_cols = ncols - self.cols;
        let change_rows = nrows - self.rows;

        if change_rows + change_cols == 0 {
            return;
        }
    }

    fn flush(&mut self) {
        self.out.queue(SetBackgroundColor(self.background)).unwrap();
        self.out.queue(SetForegroundColor(self.foreground)).unwrap();
        self.out.flush().unwrap();
    }

    fn place(&mut self) {
        let c = random_range(0..self.cols as usize);
        let r = random_range(0..self.rows as usize);
        let mut char = CHARSET[random_range(0..CHARSET.len())];

        if self.buffer[c][r] != ' ' {
            char = ' '
        }
        self.buffer[c][r] = char;
    }

    fn write(&mut self) {
        for c in 0..self.cols {
            for r in 0..self.rows {
                self.out.queue(cursor::MoveTo(c, r)).unwrap();
                self.out
                    .queue(Print(self.buffer[c as usize][r as usize]))
                    .unwrap();
            }
        }
    }

    pub fn run(&mut self) {
        loop {
            self.handle_resize();
            self.handle_events();
            self.place();
            self.write();
            self.flush();
        }
    }
}

impl Drop for Matrix {
    fn drop(&mut self) {
        self.cleanup();
    }
}
