// Imports

mod config;
mod mine;

use crate::config::Config;

use std::{ env, process, thread };
use std::sync::mpsc;

// Run function selector miner

fn main() {
    // Get command line arguments

    println!();
    println!("Initializing solidity function miner...");
    let config = Config::new(env::args()).unwrap_or_else(|error| {
        eprintln!("Error parsing arguments: {error}\n");
        eprintln!("{}", env::args().next().unwrap());
        eprintln!("    <function name>         Name of Solidity function to mine selectors for");
        eprintln!("    <function parameters>   Parameter list of function without spaces or abbreviations");
        eprintln!("    <zero byte target>      Number of zero bytes to mine");
        eprintln!("    <thread count>          Number of threads to mine on");
        eprintln!("    --release               Run in release mode (optimized)");
        eprintln!();
        process::exit(1);
    });
    println!(
        "Mining {}{} for {} target zero bytes with {} threads",
        config.name, config.params, config.target, config.threads
    );

    // Start mining threads

    let (sender, receiver) = mpsc::channel();
    for thread_id in 0..config.threads {
        let sender = sender.clone();
        let config = config.clone();
        thread::spawn(move || {
            mine::mine_selector(thread_id, sender, config);
        });
    }

    for message in receiver {
        println!("{message}");
    }
}