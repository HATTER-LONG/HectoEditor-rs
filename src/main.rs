use std::io::{self, stdout, Read};

use termion::raw::IntoRawMode;

fn to_ctrl_byte(c: char) -> u8 {
    let byte: u8 = c as u8;
    byte & 0b0001_1111
}

fn die(e: std::io::Error) {
    panic!("{}", e);
}

fn main() {
    // Call stdout into_raw_mod to chang read bescause terminals have their state controlled by the writer, not reader.
    // The raw mode will reset when _stdout destory, so need keep it event never use it.
    let _stdout = stdout().into_raw_mode().unwrap();

    for b in io::stdin().bytes() {
        // use match to catch error arms and deal it by die func to clean screen
        match b {
            Ok(b) => {
                let c = b as char;

                if c.is_control() {
                    println!("{:?} \r", b);
                } else {
                    println!("{:?} ({})\r", b, c);
                }
                // \r (Carriage Return) at the end of each line,
                // make sure our output is neatly printed line by line without indentation.
                if b == to_ctrl_byte('q') {
                    break;
                }
            }
            Err(err) => die(err),
        }
    }
}
