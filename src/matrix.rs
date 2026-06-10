use crate::Args;
use crate::logger::Logger;
use crate::pattern::Pattern;
use crossterm::event::{Event, KeyCode};
use crossterm::style::{Color, Print, SetBackgroundColor};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{QueueableCommand, cursor, event, execute, terminal};
use rand::{random_bool, random_range};
use std::io::{Stdout, Write, stdout};
use std::ops::{Add, Index};
use std::time::{Duration, Instant};

pub static CHARSET: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b',
    'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u',
    'v', 'w', 'x', 'y', 'z', '@', '#', '$', '%', '&', '*', '+', '-', '=', '?', '!', '<', '>',
];

pub struct Matrix {
    out: Stdout,
    cols: usize,
    rows: usize,
    patterns: Vec<Pattern>,
    background: Color,
    logger: Logger,

    timer: Duration,
    last_tick: Instant,

    speed: Duration,
    pattern_spawn_rate: f64,

    args: Args,
}

impl Matrix {
    pub fn new(args: Args) -> Self {
        let (cols, rows) = terminal::size().unwrap();
        let cols = cols as usize;
        let rows = rows as usize;
        terminal::enable_raw_mode().unwrap();
        execute!(stdout(), EnterAlternateScreen, cursor::Hide).unwrap();

        Self {
            out: stdout(),
            patterns: vec![],
            cols,
            rows,
            background: Color::Black,
            logger: Logger::new(),

            timer: Duration::ZERO,
            last_tick: Instant::now(),

            speed: Duration::from_millis(args.speed),
            pattern_spawn_rate: args.spawn,

            args,
        }
    }

    fn cleanup(&mut self) {
        execute!(stdout(), LeaveAlternateScreen, cursor::Show).unwrap();
        terminal::disable_raw_mode().unwrap();
    }

    fn clear(&mut self) {
        self.out
            .queue(terminal::Clear(terminal::ClearType::All))
            .unwrap();
    }

    fn handle_events(&mut self) {
        if event::poll(std::time::Duration::from_millis(0)).unwrap() {
            match event::read().unwrap() {
                Event::Key(key) => {
                    if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                        self.cleanup();
                        std::process::exit(0);
                    }
                }
                Event::Resize(ncols, nrows) => {
                    self.cols = ncols as usize;
                    self.rows = nrows as usize;
                    self.clear();
                }
                _ => {}
            }
        }
    }

    fn flush(&mut self) {
        self.out.flush().unwrap();
    }

    fn add_pattern(&mut self) {
        self.patterns
            .push(Pattern::new(&self.args, self.cols, self.rows));
    }

    fn draw_patterns(&mut self) {
        for p in &mut self.patterns {
            p.draw(&mut self.out, self.cols, self.rows);
        }
    }

    fn mv_patterns(&mut self) {
        for p in &mut self.patterns {
            p.mv(&mut self.out);
        }
    }

    fn cull(&mut self) {
        self.patterns
            .retain(|p| p.row.saturating_sub(p.length) <= self.rows);
    }

    fn tick(&mut self) {
        let now = Instant::now();
        let dt = now - self.last_tick;
        self.last_tick = now;
        self.timer = self.timer.add(dt);
    }

    fn draw(&mut self) {
        self.draw_patterns();
    }

    pub fn run(&mut self) {
        self.out.queue(SetBackgroundColor(self.background)).unwrap();
        self.clear();
        loop {
            self.handle_events();
            self.tick();

            if self.timer > self.speed {
                self.timer = Duration::ZERO;
                self.mv_patterns();
                if random_bool(self.pattern_spawn_rate.min(1.).max(0.)) {
                    for _ in 0..self.pattern_spawn_rate.ceil() as i32 {
                        self.add_pattern();
                    }
                }
                self.cull();
            }

            self.draw();
            self.flush();
        }
    }
}

impl Drop for Matrix {
    fn drop(&mut self) {
        self.cleanup();
    }
}
