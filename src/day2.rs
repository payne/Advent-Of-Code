
use std::cmp::{max, min};
use std::ops::Add;
use std::str::FromStr;

static INPUT: &'static str = include_str!("day2_input.txt");

struct Dimension {
    width: u32,
    height: u32,
    depth: u32,
}

impl Dimension {
    fn area(&self) -> u32 {
        let wh = self.width * self.height;
        let wd = self.width * self.depth;
        let hd = self.height * self.depth;
        2 * (wh + wd + hd) + min(min(wh, wd), hd)
    }

    fn smallest_wrap(&self) -> u32 {
        if self.width > max(self.height, self.depth) {
            (self.height + self.depth) * 2
        } else if self.height > self.depth {
            (self.width + self.depth) * 2
        } else {
            (self.width + self.height) * 2
        }
    }

    fn volume(&self) -> u32 {
        self.width * self.height * self.depth
    }

    fn ribbon_length(&self) -> u32 {
        self.smallest_wrap() + self.volume()
    }
}

pub fn main() {
    let dimensions = parse(INPUT);
    println!("Total Area: {:?}", total_area(&dimensions));
    println!("Total Ribbon Length: {:?}", total_ribbon(&dimensions));
}

fn total_ribbon(dimensions: &[Dimension]) -> u32 {
    dimensions.iter().map(Dimension::ribbon_length).fold(0, Add::add)
}

fn total_area(dimensions: &[Dimension]) -> u32 {
    dimensions.iter().map(Dimension::area).fold(0, Add::add)
}

fn parse(input: &str) -> Vec<Dimension> {
    input.lines().map(|line| Dimension::from_str(line).unwrap()).collect()
}

impl FromStr for Dimension {
    type Err = <u32 as FromStr>::Err;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut iter = input.trim().split('x').map(|i| i.parse::<u32>());
        Ok(Dimension {
            width: try!(iter.next().unwrap()),
            height: try!(iter.next().unwrap()),
            depth: try!(iter.next().unwrap()),
        })
    }
}