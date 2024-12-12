use anyhow::Result;
use std::time::Instant;

// The most basic template for this project.

fn part_one(_filepath: &str) -> Result<i32> {
    Ok(0)
}

fn part_two(_filepath: &str) -> Result<i32> {
    Ok(0)
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
        let _filepath = "part_one_test_input.txt";
        assert!(true); // This is a placeholder
    }

    #[test]
    fn test_part_two() {
        let _filepath = "part_two_test_input.txt";
        assert!(true); // This is a placeholder
    }
}
