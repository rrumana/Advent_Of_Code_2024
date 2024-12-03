use anyhow::Result;

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

    match part_one(filepath) {
        Ok(answer) => println!("Day 4, Part one answer: {}", answer),
        Err(e) => println!("Error: Could not calculate part one answer. {}", e),
    };

    match part_two(filepath) {
        Ok(answer) => println!("Day 4, Part two answer: {}", answer),
        Err(e) => println!("Error: Could not calculate part two answer. {}", e),
    };
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
