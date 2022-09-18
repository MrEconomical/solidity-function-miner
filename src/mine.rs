// Imports

use crate::config::{ SALT_LEN, RANDOM_LEN, CHAR_RANGE };
use crate::args::Args;

use std::str;
use std::sync::mpsc;

use tiny_keccak::{ Hasher, Keccak };

// Mine function selectors with zero byte target and salt

pub fn mine_selector(thread_id: u32, sender: mpsc::Sender<String>, args: Args, salt: [u8; SALT_LEN]) {
    // Get function byte vector and fill random slots

    let mut bytes = get_bytes(&args, &salt);
    let rand_start = args.name.len() + 1;
    let random_slice = (rand_start + SALT_LEN, rand_start + SALT_LEN + RANDOM_LEN);

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

// Convert function name and params with salt to byte vector

fn get_bytes(args: &Args, salt: &[u8; SALT_LEN]) -> Vec<u8> {
    let name_len = args.name.len();
    let bytes = name_len + 1 + SALT_LEN + RANDOM_LEN + args.params.len();
    let mut bytes: Vec<u8> = vec![0; bytes];

    bytes[..name_len].clone_from_slice(args.name.as_bytes());
    bytes[name_len] = b'_';
    bytes[name_len + 1..name_len + 1 + SALT_LEN].clone_from_slice(salt);
    bytes[name_len + 1 + SALT_LEN + RANDOM_LEN..].clone_from_slice(args.params.as_bytes());

    bytes
}