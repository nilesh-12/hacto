// Task List
// Task 1. Reading user input (key presses)

// Sub-Task for Task 1
// 1. Raw mode instead of canonical mode (enter sends input to program)
//     using termion (external crate) to allow enter raw mode.

use std::io::{self, stdout, Read};
// #task.1.1

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();

    for key in io::stdin().keys() {
        // #task.1.2
        match key {
            // #task.1.3
            Ok(key) => match key {
                Key::Char(c) => {
                    if c.is_control() {
                        println!("{:?}\r", c as u8);
                        // `{:?}` mean we don't know the string representation.
                    } else {
                        println!("{:?} ({})\r", c as u8, c);
                        // `{}` mean it's placeholder for the string.
                    }
                    // `\r` moves th cursor to the beginning of the line.
                }

                Key::Ctrl('q') => break,
                _ => println!("{:?}\r", key),
            },

            Err(err) => die(err),
        }
    }
}
