/// This file contains the `main` program logic which right now parses a Bochs
/// image, loads that image into memory, and starts executing it

mod err;
mod elf;
mod loader;
mod context;

use chrono;
use loader::load_bochs;
use context::start_bochs;

#[macro_export]
macro_rules! prompt {
    () => ({
        let timestamp = chrono::Local::now().format("%H:%M:%S");
        print!("\x1b[1;33m[{}]\x1b[0m", timestamp);
        print!(" \x1b[1;36mlucid\x1b[0m\x1b[1;35m>\x1b[0m\n");
    });
    ($($arg:tt)*) => ({
        let timestamp = chrono::Local::now().format("%H:%M:%S");
        print!("\x1b[1;33m[{}]\x1b[0m", timestamp);
        print!(" \x1b[1;36mlucid\x1b[0m\x1b[1;35m>\x1b[0m ");
        println!($($arg)*);
    });
}

#[macro_export]
macro_rules! fatal {
    ($err:expr) => {
        {
            print!("\x1b[1;31mFATAL:\x1b[0m ");
            $err.display();
            std::process::exit(-1);
        }
    };
}

fn main() {
    // Load Bochs into our process space
    prompt!("Loading Bochs...");
    let bochs = load_bochs().unwrap_or_else(|error| {
        fatal!(error);
    });
    prompt!("Bochs loaded {{ Entry: 0x{:X}, RSP: 0x{:X} }}",
        bochs.entry, bochs.rsp);

    // Start executing Bochs
    start_bochs(bochs); 
}
