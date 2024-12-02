use anyhow::Result;


fn parse_line(line: &str) -> Result<Vec<i32>> {
    let data = line.split(" ")
        .map(|s| match s.parse::<i32>() {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Error: Could not parse &str: {} into i32: {}", s, e);
                -1
            }
        })
        .collect::<Vec<i32>>();

    Ok(data)
}

fn prep_data(filepath: &str) -> Result<Vec<Vec<i32>>> {
    let list: Vec<Vec<i32>> = std::fs::read_to_string(filepath)?
        .lines()
        .map(|line| match parse_line(line) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Error: Could not parse line: {} into Vec: {}", line, e);
                vec![]
            }
        })
        .collect::<Vec<Vec<i32>>>();

    Ok(list)
}

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

fn part_two(_filepath: &str) -> Result<i32> {
    Ok(0)
}

fn main() {
    let filepath = "input.txt";

    println!("Hello, Advent of Code 2024!");

    match part_one(filepath) {
        Ok(answer) => println!("Day 2, Part one answer: {}", answer),
        Err(e) => println!("Error: Could not calculate part one answer. {}", e),
    };

    match part_two(filepath) {
        Ok(answer) => println!("Day 2, Part two answer: {}", answer),
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
        let _filepath = "part_two_test_input.txt";
        assert!(true); // This is a placeholder
    }
}
