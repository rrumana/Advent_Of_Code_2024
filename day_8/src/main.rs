use anyhow::Result;
use std::time::Instant;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Sub};
use std::collections::HashSet;
use std::collections::HashMap;
use itertools::Itertools;

// convenience structs

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Coordinate {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Coordinate {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn check_bounds(&self, width: usize, height: usize) -> bool {
        self.x >= 0 && self.x < width as i32 && self.y >= 0 && self.y < height as i32
    }
}

// data cleaning functions

fn parse_data(filepath: &str) -> Result<HashMap<char, Vec<Coordinate>>> {
    let file = File::open(filepath)?;
    let mut map: HashMap<char, Vec<Coordinate>> = HashMap::new();
    for (row, line) in BufReader::new(file).lines().enumerate() {
        for (col, c) in line?.char_indices() {
            if c.is_alphanumeric() {
                map.entry(c)
                    .or_default()
                    .push(Coordinate::new(col as i32, row as i32));
            }
        }
    }
    Ok(map)   
} 

fn get_dimensions(filepath: &str) -> Result<(usize, usize)> {
    let file = File::open(filepath)?;
    let mut width = 0;
    let mut height = 0;
    for line in BufReader::new(file).lines() {
        let line = line?;
        width = line.len();
        height += 1;
    }
    Ok((width, height))
}

// Part 1 Functions

fn get_antinodes(map: &HashMap<char, Vec<Coordinate>>, width: &usize, height: &usize, all: bool) -> Result<HashSet<Coordinate>> {
    let mut antinodes = HashSet::new();
    for positions in map.values() {
        for pair in positions.iter().combinations(2) {
            let (a, b) = (*pair[0], *pair[1]);
            let dist = b - a;

            if all { antinodes.insert(b); }
            let mut node = b + dist;
            while node.check_bounds(*width, *height) {
                antinodes.insert(node);
                node = node + dist;
                if !all { break }
            }

            if all { antinodes.insert(a); }
            let mut node = a - dist;
            while node.check_bounds(*width, *height) {
                antinodes.insert(node);
                node = node - dist;
                if !all { break }
            }
        }
    }
    Ok(antinodes)
}

fn part_one(filepath: &str) -> Result<usize> {
    let map = parse_data(filepath)?;
    let (width, height) = get_dimensions(filepath)?;
    let count = get_antinodes(&map, &width, &height, false)?.len();
    Ok(count)
}

// Part 2 Functions



fn part_two(filepath: &str) -> Result<usize> {
    let map = parse_data(filepath)?;
    let (width, height) = get_dimensions(filepath)?;
    let count = get_antinodes(&map, &width, &height, true)?.len();
    Ok(count)
}

fn main() {
    let filepath = "input.txt";

    println!("Hello, Advent of Code 2024!");
    
    let now = Instant::now();

    match part_one(filepath) {
        Ok(answer) => println!("Day 0, Part 1 answer: {}", answer),
        Err(e) => println!("Error: Could not calculate part one answer. {}", e),
    };

    println!("Day 0, Part 1 time elapsed {:.2?}", now.elapsed());
    let now = Instant::now();

    match part_two(filepath) {
        Ok(answer) => println!("Day 0, Part 2 answer: {}", answer),
        Err(e) => println!("Error: Could not calculate part two answer. {}", e),
    };

    println!("Day 0, Part 2 time elapsed {:.2?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }

    #[test]
    fn test_part_one() {
        let filepath = "part_one_test_input.txt";
        assert_eq!(part_one(filepath).unwrap(), 14);
    }

    #[test]
    fn test_part_two() {
        let filepath = "part_one_test_input.txt";
        assert_eq!(part_two(filepath).unwrap(), 34);
    }
}
