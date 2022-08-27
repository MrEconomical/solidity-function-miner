// Imports

use std::{ env, process, thread };
use std::sync::mpsc;

use rand::thread_rng;
use tiny_keccak::Sha3;

// Mining config parameters

struct Config {
    name: String,
    params: String,
    target: u32,
    threads: u32
}

impl Config {
    // Parse command line arguments

    fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // Get function name

        args.next();
        let name = args.next()
            .ok_or("no function name specified")?;
        
        // Get function parameters

        let params = match args.next() {
            Some(arg) => {
                if !arg.starts_with("(") || !arg.ends_with(")") {
                    return Err("expected function parameters in format (type1,type2,...)");
                } else if arg.contains(char::is_whitespace) {
                    return Err("function parameters must be in format (type1,type2,...) without spaces");
                }
                arg
            },
            None => return Err("no function parameters specified")
        };

        // Get zero byte target

        let target = match args.next() {
            Some(arg) => {
                match arg.parse::<u32>() {
                    Ok(value) => {
                        if value > 4 {
                            return Err("zero byte target must be less than or equal to 4");
                        }
                        value
                    },
                    Err(_) => return Err("could not parse zero byte target as integer")
                }
            },
            None => return Err("no zero byte target specified")
        };

        // Get thread count

        let threads = match args.next() {
            Some(arg) => {
                match arg.parse::<u32>() {
                    Ok(value) => {
                        if value == 0 {
                            return Err("thread count must be larger than 0")
                        }
                        value
                    },
                    Err(_) => return Err("could not parse thread count as integer")
                }
            },
            None => return Err("no thread count specified")
        };

        Ok(Config { name, params, target, threads })
    }
}

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
}
