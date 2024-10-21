use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, terminal,
    style::{Color, SetForegroundColor, ResetColor},
};
use std::io::{stdout, Write};

struct Editor {
    cursor_x: usize,
    cursor_y: usize,
    buffer: Vec<String>,
    mode: Mode,
}

enum Mode {
    Normal,
    Insert,
}

impl Editor {
    fn new() -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            buffer: vec![String::new()],
            mode: Mode::Normal,
        }
    }

    fn run(&mut self) {
        terminal::enable_raw_mode().expect("Failed to enable raw mode");
        let mut stdout = stdout();

        loop {
            self.render(&mut stdout);
            if !self.handle_input() {
                break;
            }
        }

        terminal::disable_raw_mode().expect("Failed to disable raw mode");
    }

    fn render(&self, stdout: &mut std::io::Stdout) {
        execute!(stdout, terminal::Clear(terminal::ClearType::All), cursor::MoveTo(0, 0)).expect("Failed to clear terminal");

        for (i, line) in self.buffer.iter().enumerate() {
            if i == self.cursor_y {
                execute!(
                    stdout,
                    SetForegroundColor(Color::Yellow),
                    cursor::MoveTo(0, i as u16)
                ).expect("Failed to set cursor position and color");
                writeln!(stdout, "{}", line).expect("Failed to write to stdout");
                execute!(stdout, ResetColor).expect("Failed to reset color");
            } else {
                writeln!(stdout, "{}", line).expect("Failed to write to stdout");
            }
        }

        execute!(
            stdout,
            cursor::MoveTo(self.cursor_x as u16, self.cursor_y as u16),
            cursor::Show
        ).expect("Failed to move cursor and show it");

        stdout.flush().expect("Failed to flush stdout");
    }

    fn handle_input(&mut self) -> bool {
        if event::poll(std::time::Duration::from_millis(500)).expect("Failed to poll for events") {
            if let Event::Key(KeyEvent { code, .. }) = event::read().expect("Failed to read event") {
                match self.mode {
                    Mode::Normal => self.handle_normal_mode(code),
                    Mode::Insert => self.handle_insert_mode(code),
                }
            }
        }
        true
    }

    fn handle_normal_mode(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char('i') => self.mode = Mode::Insert,
            KeyCode::Char('h') => {
                if self.cursor_x > 0 {
                    self.cursor_x -= 1;
                }
            }
            KeyCode::Char('l') => {
                if self.cursor_x < self.buffer[self.cursor_y].len() {
                    self.cursor_x += 1;
                }
            }
            KeyCode::Char('j') => {
                if self.cursor_y < self.buffer.len() - 1 {
                    self.cursor_y += 1;
                    self.cursor_x = self.buffer[self.cursor_y].len().min(self.cursor_x);
                }
            }
            KeyCode::Char('k') => {
                if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                    self.cursor_x = self.buffer[self.cursor_y].len().min(self.cursor_x);
                }
            }
            KeyCode::Char('q') => std::process::exit(0),
            _ => {}
        }
    }

    fn handle_insert_mode(&mut self, code: KeyCode) {
        match code {
            KeyCode::Esc => self.mode = Mode::Normal,
            KeyCode::Backspace => {
                if self.cursor_x > 0 {
                    self.buffer[self.cursor_y].remove(self.cursor_x - 1);
                    self.cursor_x -= 1;
                }
            }
            KeyCode::Enter => {
                let new_line = self.buffer[self.cursor_y].split_off(self.cursor_x);
                self.buffer.insert(self.cursor_y + 1, new_line);
                self.cursor_y += 1;
                self.cursor_x = 0;
            }
            KeyCode::Char(c) => {
                self.buffer[self.cursor_y].insert(self.cursor_x, c);
                self.cursor_x += 1;
            }
            _ => {}
        }
    }
}

fn main() {
    let mut editor = Editor::new();
    editor.run();
}
