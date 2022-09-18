// Imports

mod args;
mod mine;

use crate::args::Args;

use std::{ env, process, thread };
use std::sync::mpsc;

// Run function selector miner

fn main() {
    // Get command line arguments

    println!();
    println!("Initializing solidity function miner...");
    println!("Note: running directly from release executable is much faster than running via cargo run");
    let args = Args::new(env::args()).unwrap_or_else(|error| {
        eprintln!("Error parsing arguments: {error}");
        eprintln!("Usage for {}", env::args().next().unwrap());
        eprintln!("    <function name>         Name of Solidity function to mine selectors for");
        eprintln!("    <function parameters>   Parameter list of function without spaces or abbreviations");
        eprintln!("    <zero byte target>      Number of zero bytes to mine");
        eprintln!("    <thread count>          Number of threads to mine on");
        eprintln!();
        process::exit(1);
    });
    println!(
        "Mining {}{} for {} target zero bytes with {} threads",
        args.name, args.params, args.target, args.threads
    );

    // Start mining threads

    let (sender, receiver) = mpsc::channel();
    for thread_id in 0..args.threads {
        let sender = sender.clone();
        let args = args.clone();
        thread::spawn(move || {
            mine::mine_selector(thread_id, sender, args);
        });
    }

    for message in receiver {
        println!("{message}");
    }
}