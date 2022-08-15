use std::io::{self, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::Terminal;

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn run(&mut self) {
        // Call stdout into_raw_mod to chang read bescause
        // terminals have their state controlled by the writer, not reader.
        // The raw mode will reset when _stdout destory, so need keep it even never use it.
        let _stdout = stdout().into_raw_mode().unwrap();

        loop {
            if let Err(error) = self.refresh_screen() {
                die(&error);
            }

            if self.should_quit {
                break;
            }

            if let Err(error) = self.process_keypress() {
                die(&error);
            }
        }
    }
    // not called on an already-existing Editor instance, indicated by the missing
    // `&self` param in the function signature, this is called a static method.
    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal."),
        }
    }
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        // Writing a escape sequence to the terminal.

        // \1xb: is the escape character or 27 in decimal.
        // [: is the first character after excape character in escape sequence, it's instruct the
        //      terminal to do various text formatting task.

        // 2J: J is command(Erase In Display) to clean the screen.Escape sequence commands take arguments,
        //      which come before the command.
        //      In this case the argument is 2, which says to clear the entire screen.
        //  \x1b[1J would clear the screen up to where the cursor is,
        //      and \x1b[0J would clear the screen from the cursor up to the end of the screen.

        //print!("\x1b[3J");

        // termion already supply refresh screen
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));

        if self.should_quit {
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            print!("{}", termion::cursor::Goto(1, 1));
        }
        io::stdout().flush()
    }
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = read_key()?;
        if let Key::Ctrl('k') = pressed_key {
            self.should_quit = true;
        }
        Ok(())
    }
    fn draw_rows(&self) {
        // \r (Carriage Return) at the end of each line,
        // make sure our output is neatly printed line by line without indentation.
        for _ in 0..self.terminal.size().height {
            println!("~\r");
        }
    }
}

fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}

fn die(e: &std::io::Error) {
    print!("{}", termion::clear::All);
    panic!("{}", e);
}
