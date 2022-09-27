use std::io::{self, stdout};
// #task.1.1
use termion::raw::IntoRawMode;
// #task.1.3
use termion::event::Key;
use termion::input::TermRead;

pub struct Editor {}

impl Editor {
    pub fn run(&self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        // #seperate#evaluation
        loop {
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }


    }

    // #seperate#evaluation
    pub fn default() -> Self {
        Self {}
    }

    // #seperate#evaluation
    fn process_keypress(&self) -> Result<(), std::io::Error> {
        let pressed_key = read_key()?;
        match pressed_key {
            Key::Ctrl('q') => panic!("Program end"),
            _ => (),
        }

        Ok(())
    }
}

// #seperate#evaluation
fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}
// #task.1.2
fn die(e: std::io::Error) {
    panic!("{}", e);
}
