use std::io::{self, stdout, Write};

use termion::{
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};

use crate::Position;

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1,
            },

            // Call stdout into_raw_mod to chang read bescause
            // terminals have their state controlled by the writer, not reader.
            // The raw mode will reset when _stdout destory, so need keep it even never use it.
            _stdout: stdout().into_raw_mode()?,
        })
    }
    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
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
        print!("{}", termion::clear::All);
    }

    pub fn cursor_position(position: &Position) {
        let Position { mut x, mut y } = position;
        // saturating_add avod overflow
        x = x.saturating_add(1);
        y = y.saturating_add(1);
        let x = x as u16;
        let y = y as u16;
        print!("{}", termion::cursor::Goto(x, y));
    }

    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }

    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }

    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }
}
