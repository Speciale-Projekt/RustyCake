extern crate notify;
extern crate libafl;

use std::borrow::Borrow;
use std::fs;
use libafl::{
    bolts::AsSlice,
    inputs::{BytesInput, HasTargetBytes},
};
use notify::{Watcher, RecursiveMode, watcher, raw_watcher, RawEvent, op};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use crate::libafl::inputs::Input;

fn main() {
    monitor_file("test");
}

fn monitor_file(dir_path: &str){
    let mut filename: String = dir_path.to_owned();
    filename.push_str("/child.bin");

    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = raw_watcher(tx).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(dir_path, RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(RawEvent{path: Some(path), op: Ok(op),cookie}) =>{
                match op {
                     op::WRITE=> {
                         let contents = fs::read(filename.borrow()).expect("Something went wrong reading the file");
                    },
                    _ => {}
                }
            },
            Ok(event) => println!("{:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

