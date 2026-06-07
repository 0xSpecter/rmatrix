use std::io::Stdout;

use crate::{Args, matrix::CHARSET};
use crossterm::{
    QueueableCommand, cursor,
    style::{Color, Print, SetForegroundColor},
};
use rand::random_range;

pub static MIN_LENGTH: usize = 5;
pub static MAX_LENGTH: usize = 30;
pub static COLORS: &[Color] = &[
    Color::Red,
    Color::Green,
    Color::Yellow,
    Color::Blue,
    Color::Magenta,
    Color::Cyan,
    Color::White,
    Color::Grey,
    Color::DarkRed,
    Color::DarkGreen,
    Color::DarkYellow,
    Color::DarkBlue,
    Color::DarkMagenta,
    Color::DarkCyan,
];

#[derive(PartialEq)]
pub enum Style {
    SetCharacters,
    StaticCharacters,
    Lambda,
}

#[derive(PartialEq)]
pub enum Apperance {
    SingleColor(Color),
    DuoColor(Color, Color),
    StaticRainbow,
    RandomRainbow,
}

pub struct Pattern {
    pub col: usize,
    pub row: usize,
    pub chars: Vec<char>,
    pub length: usize,

    pub color: Vec<Color>,
    pub style: Style,
    pub apperance: Apperance,
}

impl Pattern {
    pub fn new(args: &Args, cols: usize, rows: usize) -> Self {
        let length = random_range(MIN_LENGTH..MAX_LENGTH);
        // generates only even numbers, this gives a gap between each column
        let col = random_range(0..cols / 2) * 2;

        let apperance = if args.rainbow {
            Apperance::RandomRainbow
        } else if args.static_rainbow {
            Apperance::StaticRainbow
        } else if let Some(duo) = &args.duo {
            let pri = parse_color(&duo[0]).unwrap_or(Color::DarkGreen);
            let sec = parse_color(&duo[1]).unwrap_or(Color::Cyan);
            Apperance::DuoColor(pri, sec)
        } else if let Some(color) = &args.color {
            Apperance::SingleColor(parse_color(color).unwrap_or(Color::DarkGreen))
        } else {
            Apperance::SingleColor(Color::DarkGreen)
        };

        let style = if args.set_characters {
            Style::SetCharacters
        } else if args.lambda {
            Style::Lambda
        } else {
            Style::StaticCharacters
        };

        let mut p = Self {
            col,
            row: 0,
            chars: Pattern::fill(&style, length),
            length: length,

            // temp fill
            color: vec![Color::DarkGreen; length],
            style,
            apperance,
        };

        p.change_apperance();

        p
    }

    pub fn change_apperance(&mut self) {
        self.color = match self.apperance {
            Apperance::SingleColor(color) => {
                vec![color; self.length]
            }
            Apperance::DuoColor(pri, sec) => (0..self.length)
                .map(|_| if random_range(0..2) == 0 { pri } else { sec })
                .collect(),
            Apperance::RandomRainbow | Apperance::StaticRainbow => (0..self.length)
                .map(|_| COLORS[random_range(0..COLORS.len())])
                .collect(),
        };
    }

    // requires stdout to clear trail
    pub fn mv(&mut self, stdout: &mut Stdout) {
        self.row += 1;

        // erase the cell just behind the tail
        if self.row > self.length {
            let tail_row = self.row - self.length - 1;
            stdout
                .queue(cursor::MoveTo(self.col as u16, tail_row as u16))
                .unwrap();
            stdout.queue(Print(' ')).unwrap();
        }

        match self.style {
            Style::SetCharacters | Style::Lambda => (),
            Style::StaticCharacters => {
                self.chars
                    .insert(0, CHARSET[random_range(0..CHARSET.len())]);
                self.chars.truncate(self.length);

                match self.apperance {
                    Apperance::SingleColor(color) => self.color.insert(0, color),
                    Apperance::DuoColor(pri, sec) => self
                        .color
                        .insert(0, if random_range(0..2) == 0 { pri } else { sec }),
                    Apperance::StaticRainbow => {
                        self.color.insert(0, COLORS[random_range(0..COLORS.len())])
                    }
                    _ => (),
                }
                self.color.truncate(self.length);
            }
        };
    }

    pub fn draw(&mut self, stdout: &mut Stdout, cols: usize, rows: usize) {
        for i in 0..self.length {
            if let Some(row) = (self.row + i)
                .checked_sub(self.length)
                .filter(|row| row < &rows)
            {
                match self.apperance {
                    Apperance::SingleColor(_)
                    | Apperance::StaticRainbow
                    | Apperance::DuoColor(_, _) => {
                        if i == self.length - 1 {
                            stdout.queue(SetForegroundColor(Color::White)).unwrap()
                        } else {
                            stdout
                                .queue(SetForegroundColor(self.color[self.length - 1 - i]))
                                .unwrap()
                        }
                    }
                    Apperance::RandomRainbow => stdout
                        .queue(SetForegroundColor(COLORS[random_range(0..COLORS.len())]))
                        .unwrap(),
                };
                stdout
                    .queue(cursor::MoveTo(self.col as u16, row as u16))
                    .unwrap();
                stdout
                    .queue(Print(self.chars[self.length - 1 - i]))
                    .unwrap();
            }
        }
    }

    fn fill(style: &Style, length: usize) -> Vec<char> {
        let mut chars: Vec<char> = vec![];

        for _ in 0..length {
            match style {
                Style::Lambda => chars.push('λ'),
                _ => chars.push(CHARSET[random_range(0..CHARSET.len())]),
            }
        }

        chars
    }
}

fn parse_color(color: &String) -> Option<Color> {
    match color.as_str() {
        "red" => Some(Color::Red),
        "green" => Some(Color::Green),
        "yellow" => Some(Color::Yellow),
        "blue" => Some(Color::Blue),
        "magenta" => Some(Color::Magenta),
        "cyan" => Some(Color::Cyan),
        "white" => Some(Color::White),
        "grey" | "gray" => Some(Color::Grey),
        "darkred" | "dark_red" => Some(Color::DarkRed),
        "darkgreen" | "dark_green" => Some(Color::DarkGreen),
        "darkyellow" | "dark_yellow" => Some(Color::DarkYellow),
        "darkblue" | "dark_blue" => Some(Color::DarkBlue),
        "darkmagenta" | "dark_magenta" => Some(Color::DarkMagenta),
        "darkcyan" | "dark_cyan" => Some(Color::DarkCyan),
        _ => None,
    }
}
