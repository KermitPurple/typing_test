use crossterm::{
    cursor, queue,
    style::{Color, PrintStyledContent, Stylize},
};
use std::io;

const COMPLETED: Color = gray(255);
const UNCOMPLETED: Color = gray(100);
const ERROR: Color = Color::Rgb { r: 230, g: 0, b: 0 };
const LINE_LEN: usize = 10;

/// ALL of the words possible
/// taken from <https://github.com/monkeytypegame/monkeytype/blob/master/frontend/static/languages/english.json>
const WORDS: &[&str] = include!("words.txt");

/// Return a color where the r, g, and b values are set to x
/// Effectively a grayscale color
const fn gray(x: u8) -> Color {
    Color::Rgb { r: x, g: x, b: x }
}

/// Get a random word from the list of words
fn next_word() -> &'static str {
    WORDS[rand::random::<usize>() % WORDS.len()]
}

/// Get a line comprised of {LINE_LEN} random words
fn next_line() -> String {
    std::iter::repeat_with(next_word)
        .take(LINE_LEN)
        .map(|x| x.to_string())
        .reduce(|a, b| format!("{} {}", a, b))
        .unwrap_or_default()
}

/// A struct representing expected input and actual input
#[derive(Clone, Debug)]
pub struct Line {
    buffer: String,
    expected: String,
    pub index: usize,
}

impl Default for Line {
    fn default() -> Self {
        Line::new()
    }
}

impl Line {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            expected: next_line(),
            index: 0,
        }
    }

    /// Create an empty line that has no expected input
    pub fn empty() -> Self {
        Self {
            expected: String::new(),
            ..Self::new()
        }
    }

    /// Calculate the number of correctly completed words
    pub fn word_count(&self) -> u32 {
        let buffer: Vec<char> = self.buffer.chars().chain([' ']).collect();
        let expected: Vec<char> = self.expected.chars().collect();
        let mut word_correct = true;
        let mut count = 0;
        for i in 0..buffer.len() {
            if i >= expected.len() {
                if word_correct {
                    count += 1;
                }
                break;
            }
            if expected[i] == ' ' {
                if word_correct {
                    count += 1;
                }
                word_correct = true;
            }
            if buffer[i] != expected[i] {
                word_correct = false;
            }
        }
        count
    }

    /// remove one character if it exists
    pub fn backspace(&mut self) {
        if self.index > 0 {
            self.buffer.pop();
            self.index -= 1;
        }
    }

    /// Returns true if a word has been finshed
    pub fn add_char(&mut self, ch: char) {
        self.buffer.push(ch);
        self.index += 1;
    }

    /// draw the line to provided stdout
    pub fn draw(&self, stdout: &mut io::Stdout) -> crossterm::Result<()> {
        let buffer: Vec<char> = self.buffer.chars().collect();
        let expected: Vec<char> = self.expected.chars().collect();
        for i in 0..buffer.len().max(expected.len()) {
            let ch = if i >= buffer.len() {
                expected[i].with(UNCOMPLETED)
            } else if i >= expected.len() {
                buffer[i].with(ERROR)
            } else {
                let color = if buffer[i] == expected[i] {
                    COMPLETED
                } else {
                    ERROR
                };
                if buffer[i] == ' ' {
                    buffer[i].on(color)
                } else {
                    buffer[i].with(color)
                }
            };
            queue!(stdout, PrintStyledContent(ch))?;
        }
        queue!(stdout, cursor::MoveToNextLine(1))
    }

    /// return true if all of the expected input has been completed
    pub fn done(&self) -> bool {
        self.index >= self.expected.len()
    }
}
