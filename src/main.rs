// Imports

mod config;

use crate::config::Config;

use std::{ env, process, thread };
use std::sync::mpsc;

use rand::thread_rng;
use tiny_keccak::Sha3;

// Run function selector miner

fn main() {
    // Get command line arguments

    println!();
    println!("Initializing solidity function miner...");
    let config = Config::new(env::args()).unwrap_or_else(|error| {
        eprintln!("Error parsing arguments: {error}\n");
        eprintln!("cargo run");
        eprintln!("    <function name>         Name of Solidity function to mine selectors for");
        eprintln!("    <function parameters>   Parameter list of function without spaces or abbreviations");
        eprintln!("    <zero byte target>      Number of zero bytes to mine");
        eprintln!("    <thread count>          Number of threads to mine on");
        eprintln!("    --release               Run in release mode (optimized)");
        eprintln!();
        process::exit(1);
    });
    println!("Mining {}{} for {} target zero bytes with {} threads", config.name, config.params, config.target, config.threads);

    // Start mining threads

    mine_selector(config);
}

// Mine function selectors with zero byte target

fn mine_selector(config: Config) {

}