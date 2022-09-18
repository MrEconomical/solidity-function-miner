// Imports

mod config;
mod args;
mod mine;

use crate::config::{ SALT_LEN, CHAR_RANGE };
use crate::args::Args;

use std::{ env, process, thread };
use std::sync::mpsc;

use rand::distributions::{ Distribution, Uniform };

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

    // Start mining threads with salts

    let (sender, receiver) = mpsc::channel();
    let salts = get_salts(args.threads);

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

// Generate random salts for threads

fn get_salts(num: u32) -> Vec<[u8; SALT_LEN]> {
    let mut salts = Vec::with_capacity(num as usize);
    let chars = Uniform::from(CHAR_RANGE.0..CHAR_RANGE.1 + 1);
    let mut rng = rand::thread_rng();

    for _ in 0..num {
        let mut salt = [0; SALT_LEN];
        loop {
            salt.fill_with(|| chars.sample(&mut rng));
            if !salts.contains(&salt) {
                break;
            }
        }
        salts.push(salt);
    }

    salts
}