use anyhow::{Context, Result};
use std::time::Instant;
use std::fs;

// data cleaning methods

fn parse_line(line: &str) -> Result<Vec<i32>> {
    line.split(' ')
        .map(|s| {
            s.parse::<i32>()
                .with_context(|| format!("Failed to parse '{}' into i32", s))
        })
        .collect()
}

fn prep_data(filepath: &str) -> Result<Vec<Vec<i32>>> {
    fs::read_to_string(filepath)?
        .lines()
        .map(|line| parse_line(line))
        .collect()
}

// part one methods

fn is_ascending(data: &Vec<i32>) -> bool {
    for i in 0..data.len() - 1 {
        if data[i] > data[i + 1] {
            return false;
        }
    }

    true
}

fn is_descending(data: &Vec<i32>) -> bool {
    for i in 0..data.len() - 1 {
        if data[i] < data[i + 1] {
            return false;
        }
    }

    true
}

fn is_gradual(data: &Vec<i32>) -> bool {
    for i in 0..data.len() - 1 {
        let distance = data[i] - data[i+1];
        if distance.abs() < 1 || distance.abs() > 3 {
            return false;
        }
    }

    true
}

fn part_one(filepath: &str) -> Result<i32> {
    let data = prep_data(filepath)?;
    let mut num_safe = 0;

    for line in data {
        if (is_ascending(&line) || is_descending(&line)) && is_gradual(&line) {
            num_safe += 1;
        }
    }

    Ok(num_safe)
}

// part two methods

fn exclude_index<T: Copy>(data: &Vec<T>, index: usize) -> Vec<T> {
    data.iter()
        .enumerate()
        .filter(move |&(i, _)| i != index)
        .map(|(_, &item)| item)
        .collect()
}

fn make_safe(line: &Vec<i32>) -> bool {
    for i in 0..line.len() {
        let new_line = exclude_index(line, i);
        if (is_ascending(&new_line) || is_descending(&new_line)) && is_gradual(&new_line) {
            return true;
        }
    }

    false
}

fn part_two(filepath: &str) -> Result<i32> {
    let data = prep_data(filepath)?;
    let mut num_safe = 0;

    for line in data {
        if !((is_ascending(&line) || is_descending(&line)) && is_gradual(&line)) {
           if !make_safe(&line) {
               continue;
           }
        }
        num_safe += 1;
    }

    Ok(num_safe)
}

fn main() {
    let filepath = "input.txt";

    println!("Hello, Advent of Code 2024!");

    let now = Instant::now();

    match part_one(filepath) {
        Ok(answer) => println!("Day 2, Part 1 answer: {}", answer),
        Err(e) => println!("Error: Could not calculate part one answer. {}", e),
    };

    println!("Day 2, Part 1 time elapsed {:.2?}", now.elapsed());
    let now = Instant::now();

    match part_two(filepath) {
        Ok(answer) => println!("Day 2, Part 2 answer: {}", answer),
        Err(e) => println!("Error: Could not calculate part two answer. {}", e),
    };

    println!("Day 2, Part 2 time elapsed {:.2?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("7 6 4 2 1").unwrap(), vec![7, 6, 4, 2, 1]);
        assert_eq!(parse_line("1 2 7 8 9").unwrap(), vec![1, 2, 7, 8, 9]);
        assert_eq!(parse_line("9 7 6 2 1").unwrap(), vec![9, 7, 6, 2, 1]);
        assert_eq!(parse_line("1 3 2 4 5").unwrap(), vec![1, 3, 2, 4, 5]);
        assert_eq!(parse_line("8 6 4 4 1").unwrap(), vec![8, 6, 4, 4, 1]);
        assert_eq!(parse_line("1 3 6 7 9").unwrap(), vec![1, 3, 6, 7, 9]);
    }

    #[test]
    fn test_prep_data() {
        let filepath = "part_one_test_input.txt";
        assert_eq!(prep_data(filepath).unwrap(), vec![vec![7, 6, 4, 2, 1], 
                                                      vec![1, 2, 7, 8, 9],
                                                      vec![9, 7, 6, 2, 1],
                                                      vec![1, 3, 2, 4, 5],
                                                      vec![8, 6, 4, 4, 1],
                                                      vec![1, 3, 6, 7, 9]]);
    }

    #[test]
    fn test_part_one() {
        let filepath = "part_one_test_input.txt";
        assert!(part_one(filepath).unwrap() == 2);
    }

    #[test]
    fn test_part_two() {
        let filepath = "part_one_test_input.txt";
        assert!(part_two(filepath).unwrap() == 4);
    }
}
