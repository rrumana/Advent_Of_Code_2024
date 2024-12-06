use anyhow::{Context, Result};
use std::time::Instant;
use multimap::MultiMap;

// convenience structs

#[derive(Debug)]
struct Data {
    page_map: MultiMap<i32,i32>,
    page_lists: Vec<Vec<i32>>,
}

// Data cleaning functions

fn get_input(filepath: &str) -> Result<Data> {
    let raw_data = std::fs::read_to_string(filepath)?;
    let (page_data, list_data)  = raw_data.split_once("\n\n").with_context(|| "Could not split data")?;
    
    let page_map: MultiMap<i32, i32> = page_data
        .lines()
        .map(|line| {
            let (key, value) = line
                .split_once("|")
                .with_context(|| "Could not split line")?;
            let key = key
                .trim()
                .parse::<i32>()
                .with_context(|| "Could not parse the key to an integer")?;
            let value = value
                .trim()
                .parse::<i32>()
                .with_context(|| format!("Could not parse '{}' to an integer", value))?;
            Ok((key, value))
        })
        .collect::<Result<MultiMap<i32,i32>>>()?;

    let page_lists: Vec<Vec<i32>> = list_data
        .lines()
        .map(|line| {
            line.split(",")
                .map(|x| x.parse::<i32>()
                    .with_context(|| format!("Could not parse '{}' to an integer", x)))
                .collect::<Result<Vec<i32>>>()
        })
        .collect::<Result<Vec<Vec<i32>>>>()?;

    Ok(Data { page_map, page_lists })
}

// Part 1 Functions

fn check_order(page_map: &MultiMap<i32, i32>, list: &Vec<i32>) -> bool {
    let mut last = match list.last() {
        Some(x) => *x,
        None => return false,
    };

    for x in list.into_iter().rev() {
        if *x == last {
            continue;
        }
        let page_list = match page_map.get_vec(x) {
            Some(x) => x,
            None => return false,
        };
        if page_list.contains(&last) {
            last = *x;
        } else {
            return false;
        }
    }
    true
}

fn find_middle(list: &Vec<i32>) -> i32 {
    list[(list.len()-1) / 2]
}

fn part_one(filepath: &str) -> Result<i32> {
    let data = get_input(filepath)?;

    let mut sum: i32 = 0;
    for line in data.page_lists {
        if check_order(&data.page_map, &line) {
            sum += find_middle(&line);
        } 
    }

    Ok(sum)
}

// Part 2 Functions

fn order_line(page_map: &MultiMap<i32, i32>, list: &Vec<i32>) -> Result<Vec<i32>> {
    let mut ordered_list = vec![0; list.len()];

    for first in list {
        let mut counter = 0;
        let page_list = match page_map.get_vec(first) {
            Some(list) => list,
            None => {
                ordered_list[list.len()-1] = *first;
                continue;
            }
        };
        for second in list {
            if first == second {
                continue;
            }
            if page_list.contains(second) {
                counter += 1;
            }
        }
        ordered_list[counter] = *first;
    }

    Ok(ordered_list)
}

fn part_two(filepath: &str) -> Result<i32> {
    let data = get_input(filepath)?;

    let mut sum: i32 = 0;
    for line in data.page_lists {
        if !check_order(&data.page_map, &line) {
            let new_line = order_line(&data.page_map, &line)?;
            sum += find_middle(&new_line);
        } 
    }

    Ok(sum)
}

fn main() {
    let filepath = "input.txt";

    println!("Hello, Advent of Code 2024!");

    let now = Instant::now();

    match part_one(filepath) {
        Ok(answer) => println!("Day 5, Part 1 answer: {}", answer),
        Err(e) => println!("Error: Could not calculate part one answer. {}", e),
    };

    println!("Day 5, Part 1 time elapsed: {:.2?}", now.elapsed());
    let now = Instant::now();

    match part_two(filepath) {
        Ok(answer) => println!("Day 5, Part 2 answer: {}", answer),
        Err(e) => println!("Error: Could not calculate part two answer. {}", e),
    };

    println!("Day 5, Part 2 time elapsed: {:.2?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }

    #[test]
    fn test_find_middle() {
        let list = vec![1, 2, 3, 4, 5];
        assert_eq!(find_middle(&list), 3);
    }

    #[test]
    fn test_order_check() {
        let filepath = "part_one_test_input.txt";
        let data = get_input(filepath).unwrap();
        for (i, line) in data.page_lists.into_iter().enumerate() {
            if i < 3 {
                assert!(check_order(&data.page_map, &line));
            } else {
                assert!(!check_order(&data.page_map, &line));
            }
        }
    }

    #[test]
    fn test_part_one() {
        let filepath = "part_one_test_input.txt";
        assert!(part_one(filepath).unwrap() == 143);
    }

    #[test]
    fn test_part_two() {
        let _filepath = "part_two_test_input.txt";
        assert!(part_two(_filepath).unwrap() == 123); 
    }
}
