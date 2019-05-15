extern crate crossbeam;

extern crate libaudiograph;

use std::sync::{Arc, RwLock};

use crossbeam::channel::unbounded;

use libaudiograph::execution::work_stealing::run_work_stealing;
use libaudiograph::measure::Measure;
use libaudiograph::parser::audiograph::parser::parse_audio_graph;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        panic!("No files supplied");
    }

    let dag = parse_audio_graph(&args[1]).expect("Failed to parse audio graph");

    let (tx, rx) = unbounded();
    let mut measure_thread = Measure::new(rx);

    std::thread::spawn(move || {
        measure_thread.receive();
    });

    match run_work_stealing(Arc::new(RwLock::new(dag)), tx) {
        Ok(_) => {}
        e => {
            eprintln!("Failed to run because: {:?}", e);
        }
    }
}