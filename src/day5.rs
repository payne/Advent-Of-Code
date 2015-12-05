
// Ironically, I did some naughty things to determine if a string is naughty or nice.
// Most importantly, I assumed the input was ASCII. If you have multi-byte characters you will
// have a bad time.

use std::str;

static INPUT: &'static str = include_str!("input/day5.txt");

pub fn main() {
    println!("(Part 1) Nice Strings: {:?}", INPUT.lines().filter(|s| nice1(s)).count());
    println!("(Part 2) Nice Strings: {:?}", INPUT.lines().filter(|s| nice2(s)).count());
}

const VOWELS: &'static [char] = &['a', 'e', 'i', 'o', 'u'];
const BAD_PAT: &'static [&'static str] = &["ab", "cd", "pq", "xy"];

pub fn nice1(input: &str) -> bool {
    (input.split(VOWELS).count() <= 3)
    && BAD_PAT.iter().any(|pat| input.contains(pat))
    && input.as_bytes().windows(2).any(|pair| pair[0] == pair[1])
}

pub fn nice2(input: &str) -> bool {
    // Byte slices allow more fun.
    let bytes = input.as_bytes();

    bytes.windows(3).any(|pair| pair[0] == pair[2]) && {
        // Iterate through every pair of characters
        bytes.windows(2).enumerate().any(|(i, pair)|
            // Find the last occurence of the pattern in the string.
            input.rfind(str::from_utf8(pair).unwrap())
                // And make sure it's not sharing characters.
                .map(|index| index > i+1).unwrap_or(false)
        )
    }
}

// Make sure my math is right.
#[test]
fn test_nice2() {
    assert!(!nice2("aaa"));
    assert!(nice2("aaaa"));
}