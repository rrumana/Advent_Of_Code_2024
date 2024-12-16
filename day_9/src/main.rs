use anyhow::Result;
use std::time::Instant;
use std::fs;
use std::iter::repeat;
use std::collections::HashSet;

// Data cleaning functions

fn parse_data(filepath: &str) -> Result<Vec<u64>> {
    let data = fs::read_to_string(filepath)?
        .trim()
        .chars()
        .map(|c| (c as u8 - b'0') as u64)
        .collect::<Vec<u64>>();

    Ok(data)
}

// Part 1 functions

fn construct_map(disk_map: &[u64]) -> Vec<Option<u64>> {
    let mut flat = Vec::new();
    let mut file_id = 0;
    for (i, &size) in disk_map.iter().enumerate() {
        match i % 2 {
            0 => {
                flat.extend(repeat(Some(file_id)).take(size as usize));
                file_id += 1;
            },
            _ => flat.extend(repeat(None).take(size as usize)),
        }
    }
    flat
}

fn compute_checksum(flat: &[Option<u64>]) -> u64 {
    let mut result = 0u64;
    for (position, &block) in flat.iter().enumerate() {
        if let Some(id) = block {
            result += id * (position as u64);
        }
    }
    result
}


fn part_one(filepath: &str) -> Result<u64> {
    let disk_map = parse_data(filepath)?;
    let mut flat = construct_map(&disk_map);

    let mut left = 0;
    let mut right = flat.len() - 1;

    while left < right {
        while left < flat.len() && flat[left].is_some() {
            left += 1;
        }

        while right > left && flat[right].is_none() {
            right = right.saturating_sub(1);
        }

        if left >= right {
            break;
        }

        flat[left] = flat[right];
        flat[right] = None;

        left += 1;
        right = right.saturating_sub(1);
    }

    Ok(compute_checksum(&flat))
}

// Part 2 functions

fn identify_files(flat: &[Option<u64>]) -> Vec<(u64, usize, usize)> {
    let mut files = Vec::new();
    let mut seen = HashSet::new();
    let mut i = 0;
    while i < flat.len() {
        if let Some(id) = flat[i] {
            if !seen.contains(&id) {
                seen.insert(id);
                let start = i;
                while i < flat.len() && flat[i] == Some(id) {
                    i += 1;
                }
                let length = i - start;
                files.push((id, start, length));
            } else {
                i += 1;
            }
        } else {
            i += 1;
        }
    }
    files
}

fn find_free_space(flat: &[Option<u64>], limit: usize, needed: usize) -> Option<usize> {
    let mut run_start = None;
    let mut run_length = 0;

    for pos in 0..limit {
        match flat[pos] {
            None => {
                if run_start.is_none() {
                    run_start = Some(pos);
                    run_length = 1;
                } else {
                    run_length += 1;
                }
                if run_length >= needed {
                    return run_start;
                }
            }
            Some(_) => {
                run_start = None;
                run_length = 0;
            }
        }
    }

    None
}

fn part_two(filepath: &str) -> Result<u64> {
    let disk_map = parse_data(filepath)?;
    let mut flat = construct_map(&disk_map);

    let mut files = identify_files(&flat);
    files.sort_by_key(|f| std::cmp::Reverse(f.0));

    for &(id, start, length) in &files {
        if start == 0 {
            continue;
        }

        if let Some(free_start) = find_free_space(&flat, start, length) {
            for offset in 0..length {
                flat[start + offset] = None;
                flat[free_start + offset] = Some(id);
            }
        }
    }

    Ok(compute_checksum(&flat))
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

    // test the main function
    // checks if the main function runs without error in a readonable amount of time
    #[test]
    fn test_main() {
        main();
    }

    // test part one against the provided test input
    // checks if the functions returns without error and gives the correct value
    #[test]
    fn test_part_one() {
        let _filepath = "part_one_test_input.txt";
        assert_eq!(part_one(_filepath).unwrap(), 1928);
    }

    // test part two against the provided test input
    // checks if the functions returns without error and gives the correct value
    #[test]
    fn test_part_two() {
        let _filepath = "part_two_test_input.txt";
        assert_eq!(part_two(_filepath).unwrap(), 2858);
    }
}
