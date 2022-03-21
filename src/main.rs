extern crate notify;
extern crate libafl;

use libafl::{
    bolts::AsSlice,
    inputs::{BytesInput, HasTargetBytes},
};
use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use crate::libafl::inputs::Input;

fn main() {

    let mut harness = |input: &BytesInput| {
        // Create a channel to receive the events.
        let (tx, rx) = channel();

        // Create a watcher object, delivering debounced events.
        // The notification back-end is selected based on the platform.
        let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();

        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        watcher.watch("oof.tx", RecursiveMode::Recursive).unwrap();

        input.to_file("foo.txt");

        loop {
            match rx.recv() {
                Ok(event) => match event {
                    CLOSE_WRITE => {
                        let mut inputFile = File::open("oof.txt").unwrap();
                        let mut reader = BufReader::new(inputFile);
                        let mut buffer = Vec::new();

                        // Read file into vector.
                        reader.read_to_end(&mut buffer);
                        if buffer[0..1] == [0x00;0x11] {
                            panic!("123");
                        }
                    }
                    _ => continue
                },
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    };
    let mut discovery = b"\xff\x10\x1a\x04\x80\x02\x18\x00";
    let input = BytesInput::new(discovery.to_vec());
    harness(&input);

}
