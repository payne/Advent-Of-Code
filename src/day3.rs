
use std::collections::HashSet;
use std::mem;

static INPUT: &'static str = include_str!("day3_input.txt");

pub fn main() {
    println!("Recieving Presents: {:?}", receiving_presents(INPUT, false));
    println!("Recieving Presents with robot: {:?}", receiving_presents(INPUT, true));
}

pub fn receiving_presents(input: &str, robot: bool) -> usize {
    let mut visited = HashSet::new();

    let ref mut santa_pos = (0, 0);
    let ref mut swap_pos = (0, 0);

    for c in input.chars() {
        match c {
            '<' => santa_pos.0 -= 1,
            '^' => santa_pos.1 += 1,
            '>' => santa_pos.0 += 1,
            'v' => santa_pos.1 -= 1,
            c => panic!("Unknown char: {:?}", c),
        }
        visited.insert(*santa_pos);
        if robot {
            mem::swap(santa_pos, swap_pos);
        }
    }

    visited.len()
}