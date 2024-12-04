use anyhow::Result;
use std::time::Instant;

// Data cleaning methods

fn parse_data(filepath: &str) -> Result<Vec<Vec<char>>> {
    let data = std::fs::read_to_string(filepath)?;
    let output = data
        .lines()
        .map(|line| line
            .chars()
            .collect())
        .collect();

    Ok(output)
}

// Part one methods

fn part_one(filepath: &str) -> Result<i32> {
    let data = parse_data(filepath)?;
    
    let mut count = 0;
    for (row, line) in data.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            if *c == 'X' {
                if row > 2 && col > 2 
                    && data[row - 1][col - 1] == 'M' 
                    && data[row - 2][col - 2] == 'A' 
                    && data[row - 3][col - 3] == 'S' {
                    count += 1;
                }
                
                if row > 2 
                    && data[row - 1][col] == 'M' 
                    && data[row - 2][col] == 'A' 
                    && data[row - 3][col] == 'S' {
                    count += 1;
                }

                if row > 2 && col < line.len() - 3
                    && data[row - 1][col + 1] == 'M' 
                    && data[row - 2][col + 2] == 'A' 
                    && data[row - 3][col + 3] == 'S' {
                    count += 1;
                }

                if col > 2 
                    && data[row][col - 1] == 'M' 
                    && data[row][col - 2] == 'A' 
                    && data[row][col - 3] == 'S' {
                    count += 1;
                }

                if col < line.len() - 3 
                    && data[row][col + 1] == 'M' 
                    && data[row][col + 2] == 'A' 
                    && data[row][col + 3] == 'S' {
                    count += 1;
                }

                if row < data.len() - 3 && col > 2
                    && data[row + 1][col - 1] == 'M' 
                    && data[row + 2][col - 2] == 'A' 
                    && data[row + 3][col - 3] == 'S' {
                    count += 1;
                }

                if row < data.len() - 3
                    && data[row + 1][col] == 'M' 
                    && data[row + 2][col] == 'A' 
                    && data[row + 3][col] == 'S' {
                    count += 1;
                }

                if row < data.len() - 3 && col < line.len() - 3
                    && data[row + 1][col + 1] == 'M' 
                    && data[row + 2][col + 2] == 'A' 
                    && data[row + 3][col + 3] == 'S' {
                    count += 1;
                }
            }
        }
    }

    Ok(count)
}

fn part_two(filepath: &str) -> Result<i32> {
    let data = parse_data(filepath)?;
    
    let mut count = 0;
    for (row, line) in data.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            if *c == 'A' && row > 0 && row < data.len() - 1 && col > 0 && col < line.len() - 1 {
                 if data[row - 1][col - 1] == 'M' &&
                    data[row - 1][col + 1] == 'M' &&
                    data[row + 1][col - 1] == 'S' &&
                    data[row + 1][col + 1] == 'S' {
                    count += 1;
                 }

                 if data[row - 1][col - 1] == 'S' &&
                    data[row - 1][col + 1] == 'S' &&
                    data[row + 1][col - 1] == 'M' &&
                    data[row + 1][col + 1] == 'M' {
                    count += 1;
                 }
                 

                 if data[row - 1][col - 1] == 'M' &&
                    data[row - 1][col + 1] == 'S' &&
                    data[row + 1][col - 1] == 'M' &&
                    data[row + 1][col + 1] == 'S' {
                    count += 1;
                 }


                 if data[row - 1][col - 1] == 'S' &&
                    data[row - 1][col + 1] == 'M' &&
                    data[row + 1][col - 1] == 'S' &&
                    data[row + 1][col + 1] == 'M' {
                    count += 1;
                 }
            }
        }
    }

    Ok(count)
}

fn main() {
    let filepath = "input.txt";

    println!("Hello, Advent of Code 2024!");

    let now = Instant::now();

    match part_one(filepath) {
        Ok(answer) => println!("Day 4, Part one answer: {}", answer),
        Err(e) => println!("Error: Could not calculate part one answer. {}", e),
    };

    println!("Day 4, Part one time elapsed: {:.2?}", now.elapsed());
    let now = Instant::now();

    match part_two(filepath) {
        Ok(answer) => println!("Day 4, Part two answer: {}", answer),
        Err(e) => println!("Error: Could not calculate part two answer. {}", e),
    };

    println!("Day 4, Part 2 time elapsed: {:.2?}", now.elapsed());
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
        assert_eq!(part_one(filepath).unwrap(), 18); 
    }

    #[test]
    fn test_part_two() {
        let filepath = "part_two_test_input.txt";
        assert_eq!(part_two(filepath).unwrap(), 9);
    }
}
