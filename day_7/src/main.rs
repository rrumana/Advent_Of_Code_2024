use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

// data cleaning functions

fn parse_data(filepath: &str) -> Result<Vec<(i64, Vec<i64>)>> {
    let file = File::open(filepath)?;
    let data: Vec<(i64,Vec<i64>)> = BufReader::new(file)
        .lines()
        .map(|line| {
            let line = line.with_context(|| "Could not read line")?;
            let (total, parts) = line
                .split_once(":")
                .with_context(|| "Could not split line: {}, line")?;
            let total = total
                .parse::<i64>()
                .with_context(|| format!("Could not parse the total: {} to an integer", total))?;
            let parts = parts
                .split_whitespace()
                .map(|x| x.parse::<i64>()
                    .with_context(|| format!("Could not parse the part: '{}' to an integer", x)))
                .collect::<Result<Vec<i64>>>()?;
            Ok((total, parts))
        })
        .collect::<Result<Vec<(i64, Vec<i64>)>>>()?;

    Ok(data)
}

// Part 1 Functions

fn evaluate(nums: &Vec<i64>, operators: &Vec<char>) -> Result<i64> {
    let mut result = 0;
    match nums.len() {
        0 => Ok(result),
        1 => Ok(nums[0]),
        _ => {
            result = nums[0];
            for (i, num) in nums.iter().skip(1).enumerate() {
                match operators[i] {
                    '+' => result += num,
                    '*' => result *= num,
                    '|' => result = concatenate(&result, num), 
                    _ => (),
                }
            }
            Ok(result)
        }
    }
}

fn find_combo(total: &i64, nums: &Vec<i64>) -> Result<bool> {
    let mut operators = vec!['+'; nums.len() - 1];
    
    for i in 0..2i64.pow(operators.len() as u32) {
        let mut temp = i;
        for j in 0..operators.len() {
            operators[j] = match temp % 2 {
                0 => '+',
                1 => '*',
                _ => '+',
            };
            temp /= 2;
        }
        if evaluate(nums, &operators)? == *total {
            return Ok(true)
        }
    }
    Ok(false)
}

fn part_one(filepath: &str) -> Result<i64> {
    let data = parse_data(filepath)?;
    let mut result = 0;
    for (total, nums) in data {
        if find_combo(&total, &nums)? {
            result += total;
        }
    }
    Ok(result)
}

// Part 2 Functions

fn concatenate(i: &i64, j: &i64) -> i64 {
    i * 10i64.pow(j.ilog10() + 1) + j
}

fn find_combo_two(total: &i64, nums: &Vec<i64>) -> Result<bool> {
    let mut operators = vec!['+'; nums.len() - 1];
    
    for i in 0..3i64.pow(operators.len() as u32) {
        let mut temp = i;
        for j in 0..operators.len() {
            operators[j] = match temp % 3 {
                0 => '+',
                1 => '*',
                2 => '|',
                _ => '+',
            };
            temp /= 3;
        }
        if evaluate(nums, &operators)? == *total {
            return Ok(true)
        }
    }
    Ok(false)
}

fn part_two(filepath: &str) -> Result<i64> {
    let data = parse_data(filepath)?;
    let mut result = 0;
    for (total, nums) in data {
        if find_combo(&total, &nums)? {
            result += total;
            continue;
        }
        if find_combo_two(&total, &nums)? {
            result += total;
        }
    }
    Ok(result)
}

fn main() {
    let filepath = "input.txt";

    println!("Hello, Advent of Code 2024!");
    
    let now = Instant::now();

    match part_one(filepath) {
        Ok(answer) => println!("Day 7, Part 1 answer: {}", answer),
        Err(e) => println!("Error: Could not calculate part one answer. {}", e),
    };

    println!("Day 7, Part 1 time elapsed {:.2?}", now.elapsed());
    let now = Instant::now();

    match part_two(filepath) {
        Ok(answer) => println!("Day 7, Part 2 answer: {}", answer),
        Err(e) => println!("Error: Could not calculate part two answer. {}", e),
    };

    println!("Day 7, Part 2 time elapsed {:.2?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }

    #[test]
    fn test_evaluate_add() {
        let nums = vec![1, 2, 3, 4, 5];
        let operators = vec!['+', '+', '+', '+'];
        assert_eq!(evaluate(&nums, &operators).unwrap(), 15);
    }

    #[test]
    fn test_evaluate_mul() {
        let nums = vec![1, 2, 3, 4, 5];
        let operators = vec!['*', '*', '*', '*'];
        assert_eq!(evaluate(&nums, &operators).unwrap(), 120);
    }

    #[test]
    fn test_concatenate() {
        assert_eq!(concatenate(&1, &2), 12);
        assert_eq!(concatenate(&12, &3), 123);
        assert_eq!(concatenate(&123, &4), 1234);
    }

    #[test]
    fn test_evaluate_concat() {
        let nums = vec![1, 2, 3, 4, 5];
        let operators = vec!['|', '|', '|', '|'];
        assert_eq!(evaluate(&nums, &operators).unwrap(), 12345);
    }

    #[test]
    fn test_evaluate_add_mul() {
        let nums = vec![1, 2, 3, 4, 5];
        let operators = vec!['+', '*', '+', '*'];
        assert_eq!(evaluate(&nums, &operators).unwrap(), 65);
    }

    #[test]
    fn test_evaluate_add_mul_concat() {
        let nums = vec![1, 2, 3, 4, 5];
        let operators = vec!['+', '*', '|', '*'];
        assert_eq!(evaluate(&nums, &operators).unwrap(), 470);
    }

    #[test]
    fn test_part_one() {
        let filepath = "part_one_test_input.txt";
        assert_eq!(part_one(filepath).unwrap(), 3749);
    }

    #[test]
    fn test_part_two() {
        let filepath = "part_one_test_input.txt";
        assert_eq!(part_two(filepath).unwrap(), 11387);
    }
}
