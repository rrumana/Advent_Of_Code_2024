use anyhow::Result;


// data cleaning methods

fn parse_line(line: &str) -> Result<(i32, i32)> {
    line.split_once("   ")
        .ok_or_else(|| anyhow::anyhow!("Could not split line: {}", line))
        .map(|(x,y)| {
            let a = match x.parse::<i32>() {
                Ok(a) => a,
                Err(e) => {
                    eprintln!("Error: Could not parse &str: {} into i32: {}", x, e);
                    -1
                }
            };
            let b = match y.parse::<i32>() {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("Error: Could not parse &str: {} into i32: {}", y, e);
                    -1
                }
            };
            (a, b)
        })
}


fn prep_data(filepath: &str) -> Result<(Vec<i32>, Vec<i32>)> {
    let (list_one, list_two): (Vec<i32>, Vec<i32>) = std::fs::read_to_string(filepath)?
        .lines()
        .map(|line| match parse_line(line) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Error: Could not parse line: {} into Tuple: {}", line, e);
                (-1, -1)
            }
        })
        .collect::<Vec<(i32, i32)>>()
        .into_iter()
        .unzip();

    Ok((list_one, list_two))
}

// part one methods

fn difference(a: &i32, b: &i32) -> i32 {
   let diff = a - b;  
   diff.abs()
}


fn part_one(filepath: &str) -> Result<i32> {
    let (mut list_one, mut list_two) = prep_data(filepath)?;

    list_one.sort();
    list_two.sort();

    let mut sum = 0;
    for (x,  y) in list_one.iter().zip(list_two.iter()) {
        sum += difference(x, y);
    }

    Ok(sum)
}

// part two methods

fn part_two(filepath: &str) -> Result<i32> {
    let (list_one, list_two) = prep_data(filepath)?;

    let mut score = 0;
    for x in list_one.iter() {
        for y in list_two.iter() {
            if x == y {
                score += x;
            }
        }
    }

    Ok(score)
}

fn main() {
    let filepath = "input.txt";

    println!("Hello, Advent of Code 2024!");

    match part_one(filepath) {
        Ok(answer) => println!("Day 1, Part one answer: {}", answer),
        Err(e) => println!("Error: Could not calculate part one answer. {}", e),
    };

    match part_two(filepath) {
        Ok(answer) => println!("Day 1, Part two answer: {}", answer),
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
    fn test_diff() {
        assert_eq!(difference(&1, &3), 2);
        assert_eq!(difference(&2, &3), 1);
        assert_eq!(difference(&3, &3), 0);
        assert_eq!(difference(&3, &4), 1);
        assert_eq!(difference(&3, &5), 2);
        assert_eq!(difference(&4, &9), 5);
    }

    #[test]
    fn test_part_one() {
        let filepath = "part_one_test_input.txt";
        assert_eq!(part_one(filepath).unwrap(), 11);
    }

    #[test]
    fn test_part_two() {
        let filepath = "part_two_test_input.txt";
        assert_eq!(part_two(filepath).unwrap(), 31);
    }
}
