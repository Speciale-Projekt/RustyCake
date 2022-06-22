extern crate notify;
extern crate libafl;
mod utils;
use crate::utils::parser;


use std::path::PathBuf;
use std::borrow::Borrow;
use std::fs;
use notify::{Watcher, RecursiveMode, watcher, raw_watcher, RawEvent, op};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::net::UdpSocket;
use std::process::{Command, ExitStatus};
use std::sync::mpsc;
use libafl::{bolts::{current_nanos, rands::StdRand, tuples::tuple_list, AsSlice}, corpus::{InMemoryCorpus, OnDiskCorpus}, Evaluator, events::SimpleEventManager, executors::{inprocess::InProcessExecutor, ExitKind}, feedbacks::{CrashFeedback, MaxMapFeedback}, fuzzer::{Fuzzer, StdFuzzer}, generators::RandPrintablesGenerator, inputs::{BytesInput, HasTargetBytes}, monitors::SimpleMonitor, mutators::scheduled::{havoc_mutations, StdScheduledMutator}, observers::StdMapObserver, stages::mutational::StdMutationalStage, state::StdState};
use libafl::corpus::QueueCorpusScheduler;
use libafl::feedbacks::AllIsNovel;
use libafl::inputs::HasBytesVec;
use libafl::observers::TimeObserver;

fn main() {
    let (file_send,file_receive): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();
    let mut harness = new_case();
    


    // Create a tmp msg that looks like: 0xFF 0x10 0x1A 0x05 0x12 0x34 0x56 0x78 0x99
    let msg: Vec<u8> = [0xFF, 0x10, 0x1A, 0x05, 0x12, 0x34, 0x56, 0x78, 0x99,0xaa,0x1, 0xFF, 0x10, 0x1A, 0x07, 0x12, 0x34, 0x56, 0x78, 0x99].to_vec();
    let res = parser::assign_command(msg);


    println!("{:?}", res);

    let temp = "";

    let mut harness = |input: &BytesInput| {
        // Create a channel to receive the events.
        let (tx, rx) = channel();
    }
}
fn new_case() -> fn(&Vec<BytesInput>,  Receiver<Vec<u8>>) -> ExitKind {
    let harness = |inputs: &Vec<BytesInput>, file_receive: Receiver<Vec<u8>> | {
        let socket = UdpSocket::bind("127.0.0.1:10001").expect("Couldn't Bind to 127.0.0.1:34254");

        let mut child = Command::new("'../openthread/output/simulation/bin/ot-cli-ftd")
            .args(["1","--master","--dataset", "{\"Network_Key\": \"cf70867da8d41fbdb614aa9677addf9e\", \"PAN_ID\": \"0x7063\"}"])
            .spawn()
            .expect("couldn't start ot-cli-ftd");


        for input in inputs {
            socket.send_to(input.bytes(), "127.0.0.1:10001").expect("Couldn't send data");
            file_receive.recv().unwrap();
        }

        match child.try_wait().unwrap(){
            None => {}
            Some(status) => {
                    panic!("Returning with exit-status: {:?}",status);
            }
        }
        ExitKind::Ok
    };
    harness
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

