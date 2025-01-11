
use std::thread;
use std::time::Duration;
use std::io::{Write};

pub struct LoadingBar {
    steps: u8,
    message: String,
}

impl LoadingBar {
    pub fn new(message: &str, steps: u8) -> Self {
        Self {
            message: message.to_string(),
            steps,
        }
    }

    pub fn start(&self) {
        let spinner = ["⣾", "⣷", "⣯", "⣟", "⡿", "⢿", "⣻", "⣽"];
        for _ in 0..self.steps {   
            for symbol in spinner.iter() {
                print!("\r{} {}", self.message, symbol);
                std::io::stdout().flush().unwrap();
                thread::sleep(Duration::from_millis(100));
            }
        }

        println!("\r{} ✓", self.message);
    }
}     