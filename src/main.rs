mod fuzz;
mod mutator;
mod utils;

extern crate notify;

use utils::*;
use std::path::PathBuf;
use std::borrow::Borrow;
use std::{fs, thread};
use notify::{Watcher, RecursiveMode, watcher, raw_watcher, RawEvent, op};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::net::UdpSocket;
use std::process::{Command, ExitStatus};
use std::sync::mpsc;
use libafl::corpus::RandCorpusScheduler;
use rand::Rng;
use crate::mutator::Mutator;
use crate::parser::assign_command;

fn main() {
    let (file_send, file_receive): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();
    let mut file = File::open("in/sequence").expect("Could not open sequence file");
    let mut data = String::new();
    file.read_to_string(&mut data);

    let bytes = assign_command(data.into_bytes());



    thread::spawn(move || {
        let mutator: Mutator = Mutator::new();
        loop {
            let rand_num = rand::thread_rng().gen_range(0..3);
            mutator.mutate(sequence[rand_num]);
            target(sequence, file_receive.clone());
        }
    });

    thread::spawn(move ||{
       monitor_file("child.bin", file_send.clone());
    });
}

fn target(inputs: &Vec<Vec<u8>>, file_receive: Receiver<Vec<u8>>) {
    let socket = UdpSocket::bind("127.0.0.1:10001").expect("Couldn't Bind to 127.0.0.1:34254");

    let mut child = Command::new("'../openthread/output/simulation/bin/ot-cli-ftd")
        .args(["1", "--master", "--dataset", "{\"Network_Key\": \"cf70867da8d41fbdb614aa9677addf9e\", \"PAN_ID\": \"0x7063\"}"])
        .spawn()
        .expect("couldn't start ot-cli-ftd");


    for input in inputs {
        socket.send_to(input.as_bytes(), "127.0.0.1:10001").expect("Couldn't send data");
        file_receive.recv_timeout(Duration::from_millis(10)).unwrap();
    }

    match child.try_wait().unwrap() {
        None => {}
        Some(status) => {
            panic!("Returning with exit-status: {:?}", status);
        }
    }
}

fn monitor_file(dir_path: &str, sender_endpoint: Sender<Vec<u8>>) {
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
            Ok(RawEvent { path: Some(path), op: Ok(op), cookie }) => {
                match op {
                    op::WRITE => {
                        // TODO: parse contents and only send if it is attaching to parent message.
                        let contents = fs::read(filename.clone()).expect("Something went wrong reading the file");
                        sender_endpoint.send(contents).unwrap();
                    }
                    _ => {}
                }
            }
            Ok(event) => println!("{:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

