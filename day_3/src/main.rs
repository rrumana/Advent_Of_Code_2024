use anyhow::Result;
use regex::Regex;
use std::fs;

// part one functions

fn get_sum(input: &str) -> Result<i32> {
    let re = match Regex::new(r"mul\((\d+),(\d+)\)") {
        Ok(re) => re,
        Err(e) => return Err(anyhow::anyhow!("Could not create regex: {}", e)),
    };

    let mut sum = 0;

    for (_, [first, second]) in re.captures_iter(input).map(|c| c.extract()) {
        let result = first.parse::<i32>()? * second.parse::<i32>()?;
        sum += result;
    }

    Ok(sum)
}

fn part_one(filepath: &str) -> Result<i32> {
    let data: String = fs::read_to_string(filepath)?;
    let ans = get_sum(&data)?;
    
    Ok(ans)
}

// part two functions

fn part_two(filepath: &str) -> Result<i32> {
    let data: String = fs::read_to_string(filepath)?;
    let re = Regex::new(r"don't\(\)[\s\S]*?do\(\)")?;
    let data = re.replace_all(&data, "");
    let ans = get_sum(&data)?;

    Ok(ans)
}

fn main() {
    let filepath = "input.txt";

    println!("Hello, Advent of Code 2024!");

    match part_one(filepath) {
        Ok(answer) => println!("Day 3, Part one answer: {}", answer),
        Err(e) => println!("Error: Could not calculate part one answer. {}", e),
    };

    match part_two(filepath) {
        Ok(answer) => println!("Day 3, Part two answer: {}", answer),
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
        let filepath = "part_one_test_input.txt";
        assert_eq!(part_one(filepath).unwrap(), 2652);
    }
}
