// Imports

use crate::args::Args;

use std::str;
use std::sync::mpsc;

use tiny_keccak::{ Hasher, Keccak };

// Mining parameters

const RANDOM_LENGTH: usize = 12;        // Length of random characters in function name
const CHAR_RANGE: (u8, u8) = (97, 122); // Range of random characters (a-z)

// Mine function selectors with zero byte target

pub fn mine_selector(thread_id: u32, sender: mpsc::Sender<String>, args: Args) {
    // Get function byte vector and fill random slots

    let mut bytes = get_bytes(&args);
    let random_slice = (args.name.len() + 1, args.name.len() + RANDOM_LENGTH + 1);
    bytes[random_slice.0..random_slice.1].clone_from_slice(&get_random(thread_id, args.threads));

    loop {
        // Increment random slice

        #[allow(clippy::needless_range_loop)]
        for b in random_slice.0..random_slice.1 {
            if bytes[b] < CHAR_RANGE.1 {
                bytes[b] += 1;
                break;
            }
            bytes[b] = CHAR_RANGE.0;
        }

        // Check function selector hash for zero bytes

        let mut hash = [0; 32];
        let mut hasher = Keccak::v256();
        hasher.update(&bytes);
        hasher.finalize(&mut hash);

        let mut zero_bytes = 0;
        #[allow(clippy::needless_range_loop)]
        for b in 0..4 {
            if hash[b] == 0 {
                zero_bytes += 1;
            }
        }

        if zero_bytes >= args.target {
            // Display targeted selector

            let message = format!(
                "[thread {thread_id}] {} = 0x{:>02x?}{:>02x?}{:>02x?}{:>02x?}",
                str::from_utf8(&bytes).unwrap(),
                hash[0], hash[1], hash[2], hash[3]
            );
            if zero_bytes > args.target {
                sender.send(format!("\n    {message}\n")).unwrap();
            } else {
                sender.send(message).unwrap();
            }
        }
    }
}

// Convert function name and params to byte vector

fn get_bytes(args: &Args) -> Vec<u8> {
    let name_len = args.name.len();
    let bytes = name_len + RANDOM_LENGTH + 1 + args.params.len();
    let mut bytes: Vec<u8> = vec![0; bytes];

    bytes[..name_len].clone_from_slice(args.name.as_bytes());
    bytes[name_len] = b'_';
    bytes[name_len + RANDOM_LENGTH + 1..].clone_from_slice(args.params.as_bytes());

    bytes
}

// Get random byte slice from thread number to divide load

fn get_random(thread_id: u32, threads: u32) -> Vec<u8> {
    let mut slice = Vec::with_capacity(RANDOM_LENGTH);
    let num_chars = (CHAR_RANGE.1 - CHAR_RANGE.0 + 1) as u64;
    let state = num_chars.pow(RANDOM_LENGTH as u32) * thread_id as u64 / threads as u64;

    for b in 0..RANDOM_LENGTH {
        slice.push(97 + ((state / num_chars.pow(b as u32)) % num_chars) as u8);
    }
    slice
}