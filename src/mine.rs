// Imports

use crate::config::Config;

use std::ops::Range;

use rand::Rng;
use tiny_keccak::{ Hasher, Keccak };

// Mining parameters

const RANDOM_LENGTH: usize = 12;                                // Length of random characters in function name
const CHAR_RANGE: (u8, u8) = (97, 122);                         // Range of random characters (a-z)
const START_CHARS: Range<u8> = CHAR_RANGE.0..CHAR_RANGE.0 + 10; // Range of starting random characters

// Mine function selectors with zero byte target

pub fn mine_selector(config: Config) {
    // Get function byte vector and fill random slots

    let mut bytes = get_bytes(&config);
    let mut rng = rand::thread_rng();
    let random_slice = (config.name.len() + 1, config.name.len() + RANDOM_LENGTH + 1);
    bytes[random_slice.0..random_slice.1].fill_with(|| rng.gen_range(START_CHARS));

    loop {
        // Increment random slice

        for b in random_slice.0..random_slice.1 {
            if bytes[b] < CHAR_RANGE.1 {
                bytes[b] += 1;
                break;
            }
            bytes[b] = 0;
        }

        // Check function selector hash for zero bytes

        let mut hash = [0u8; 32];
        let mut hasher = Keccak::v256();
        hasher.update(&bytes);
        hasher.finalize(&mut hash);

        let mut zero_bytes = 0u32;
        for b in 0..4 {
            if hash[b] == 0 {
                zero_bytes += 1;
            }
        }
    }
}

// Convert function name and params to bytes

fn get_bytes(config: &Config) -> Vec<u8> {
    let name_len = config.name.len();
    let bytes = name_len + RANDOM_LENGTH + 1 + config.params.len();
    let mut bytes: Vec<u8> = vec![0; bytes];

    bytes[..name_len].clone_from_slice(config.name.as_bytes());
    bytes[name_len] = b'_';
    bytes[name_len + RANDOM_LENGTH + 1..].clone_from_slice(config.params.as_bytes());

    bytes
}