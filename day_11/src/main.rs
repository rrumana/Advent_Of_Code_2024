use anyhow::{Context, Result};
use std::time::Instant;
use std::fs;
use std::collections::HashMap;

// data cleaning functions

fn read_data(filepath: &str) -> Result<Vec<i64>> {
    let data = fs::read_to_string(filepath)?
        .split_whitespace()
        .map(|x| {
            x.parse::<i64>()
            .with_context(|| format!("Failed to parse number: {}", x))
        })   
        .collect::<Result<Vec<i64>>>()?;

    Ok(data)
}

// part one functions

fn count_digits(num: &i64) -> i64 {
    (num.abs() as f32).log10().floor() as i64 + 1
}

fn has_even_digits(num: i64) -> bool {
    match num {
        0 => false,
        _ => count_digits(&num) % 2 == 0,
    }
}

fn split_number(num: &i64) -> (i64, i64) {
    let mid = count_digits(&num.abs())/2;
    let divisor = 10_i64.pow(mid as u32);

    let first_half = num / divisor;
    let second_half = num % divisor;

    (first_half, second_half)
}

fn blink(
    before: i64, 
    max_depth: u32, 
    current_depth: u32, 
    known_rocks: &mut HashMap<(i64, u32), i64>
) -> i64 {
    if current_depth >= max_depth {
        return 1;
    }

    if let Some(&known_rock) = known_rocks.get(&(before, current_depth)) {
        return known_rock;
    }
    
    let result = match before {
        0 => blink(1, max_depth, current_depth + 1, known_rocks),
        n if has_even_digits(n) => {
            let (left, right) = split_number(&n);
            blink(left, max_depth, current_depth + 1, known_rocks) + blink(right, max_depth, current_depth + 1, known_rocks)
        }
        _ => blink(before * 2024, max_depth, current_depth + 1, known_rocks),
    };

    known_rocks.insert((before, current_depth), result);

    result
}

fn simulate(filepath: &str, depth: &u32) -> Result<i64> {
    let data = read_data(filepath)?;
    let mut known_rocks: HashMap<(i64, u32), i64> = HashMap::new();
    let mut sum = 0;

    for rock in data {
        sum += blink(rock, *depth, 0, &mut known_rocks)
    }

    Ok(sum)
}

fn main() {
    let filepath = "input.txt";
    let part_one_depth = 25;
    let part_two_depth = 75;

    println!("Hello, Advent of Code 2024!");
    
    let now = Instant::now();

    match simulate(filepath, &part_one_depth) {
        Ok(answer) => println!("Day 11, Part 1 answer: {}", answer),
        Err(e) => println!("Error: Could not calculate part one answer. {}", e),
    };

    println!("Day 11, Part 1 time elapsed {:.2?}", now.elapsed());
    let now = Instant::now();

    match simulate(filepath, &part_two_depth) {
        Ok(answer) => println!("Day 11, Part 2 answer: {}", answer),
        Err(e) => println!("Error: Could not calculate part two answer. {}", e),
    };

    println!("Day 11, Part 2 time elapsed {:.2?}", now.elapsed());
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
        assert_eq!(simulate(filepath, &6).unwrap(), 22);
        assert_eq!(simulate(filepath, &25).unwrap(), 55312);
    }
}
