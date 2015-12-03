
static INPUT: &'static str = include_str!("input/day1.txt");

pub fn main() {
    println!("(Part 1) Final Floor: {:?}", part1(INPUT));
    println!("(Part 2) Basement Position: {:?}", part2(INPUT));
}

fn part1(input: &str) -> i32 {
    let up = input.trim().split('(').count() as i32 - 1;
    let down = input.len() as i32 - up;
    up - down
}

fn part2(input: &str) -> usize {
    let mut pos = 0;
    for (i, c) in input.chars().enumerate() {
        if c == '(' {
            pos += 1;
        } else if pos == 0 {
            return i + 1;
        } else {
            pos -= 1;
        }
    }
    0
}
