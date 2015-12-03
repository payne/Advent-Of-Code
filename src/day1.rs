
static INPUT: &'static str = include_str!("day1_input.txt");

pub fn main() {
    println!("Final Floor: {:?}", part1(INPUT));
    println!("Basement Position: {:?}", part2(INPUT));
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
