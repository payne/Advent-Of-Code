
extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::md5::Md5;

static INPUT: &'static str = include_str!("input/day4.txt");

pub fn main() {
    println!("(Part 1) Smallest number: {:?}", find_md5_suffix(INPUT, "00000"));

    // With my input, part 2 took too long in debug mode.
    // Recommended to turn on optimisations for this one.
    skip_part2_if_debug();
}

// Why is there no debug cfg flag? (Or why does it have a weird name that isn't documented)
#[cfg(debug_assertions)]
fn skip_part2_if_debug() {
    println!("Skipping Part 2 because it takes too long in debug mode");
}

#[cfg(not(debug_assertions))]
fn skip_part2_if_debug() {
    println!("(Part 2) Smallest number: {:?}", find_md5_suffix(INPUT, "000000"));
}

pub fn find_md5_suffix(input_base: &str, start_pattern: &str) -> u32 {
    let mut hash = Md5::new();
    let mut i = 0;

    loop {
        hash.input_str(&format!("{}{:?}", input_base, i));

        if hash.result_str().starts_with(start_pattern) {
            return i;
        } else {
            hash.reset();
            i += 1;
        }
    }
}