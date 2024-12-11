use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use grid::Grid;
use std::time::Instant;
use std::collections::HashSet;

// convenience ENUMS and structs

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Guard {
    x: usize,
    y: usize,
    dir: Direction,
}

impl Guard {
    fn turn_right(&mut self) {
        self.dir = match self.dir {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn get_forward_coords(&self) -> (Option<usize>, Option<usize>) {
        match self.dir {
            Direction::Up    => (self.x.checked_sub(1), Some(self.y)),
            Direction::Down  => (Some(self.x+1), Some(self.y)),
            Direction::Left  => (Some(self.x), self.y.checked_sub(1)),
            Direction::Right => (Some(self.x), Some(self.y+1)),
        }
    }

    fn move_forward(&mut self) -> Result<()> {
        match self.get_forward_coords() {
            (Some(x), Some(y)) => {
                self.x = x;
                self.y = y;
                Ok(())
            },
            _ => Err(anyhow::anyhow!("Tried to move out of bounds too a negative coordinate")),
        }
    }
}

// data cleaning functions

fn parse_data(filepath: &str) -> Result<Grid<char>> {
    let file = File::open(filepath)?;
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .collect::<Result<_, _>>()?;

    let rows = lines.len();
    let cols = lines.get(0).map_or(0, |line| line.len());

    let mut data = Vec::with_capacity(rows * cols);
    for line in lines {
        data.extend(line.chars());
    }

    let grid = Grid::from_vec(data, rows);
    Ok(grid)
}

// part one functions

fn find_guard_start(data: &mut Grid<char>) -> Guard {
    let mut guard_cords: (usize, usize) = (0, 0);
    let mut guard_direction: Direction = Direction::Up;
    let mut guard_found = None;

    for (cord, val) in data.indexed_iter() {
        match val {
            '^' => {
                guard_found = Some((cord, Direction::Up));
                break;
            },
            '>' => {
                guard_found = Some((cord, Direction::Right));
                break;
            },
            'v' => {
                guard_found = Some((cord, Direction::Down));
                break;
            },
            '<' => {
                guard_found = Some((cord, Direction::Left));
                break;
            },
            _ => (),
        }
    }

    if let Some((cords, direction)) = guard_found {
        guard_cords = cords;
        guard_direction = direction;
        data[guard_cords] = '.';
    }

    let guard = Guard {
        x: guard_cords.0,
        y: guard_cords.1,
        dir: guard_direction,
    };

    guard
}

fn simulate(data: &Grid<char>, guard: &Guard, detect_loop: bool) -> Result<(HashSet<(usize, usize)>, bool)> {
    let rows = data.rows();
    let cols = data.cols();
    let mut guard = guard.clone();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert((guard.x, guard.y));

    let mut visited_states = HashSet::new();
    if detect_loop {
        visited_states.insert((guard.x, guard.y, guard.dir));
    }

    loop {
        let (nx, ny) = match guard.get_forward_coords() {
            (Some(x), Some(y)) => (x, y),
            _ => break
        };

        if nx >= rows || ny >= cols {
            break;
        }

        match data[(nx, ny)] {
            '#' => guard.turn_right(),
            _ => {
                guard.move_forward()?;
                visited.insert((guard.x, guard.y));
            }
        }

        if detect_loop {
            let state = (guard.x, guard.y, guard.dir);
            match visited_states.contains(&state) {
                true => return Ok((visited, true)),
                false => visited_states.insert(state),
            };
        }
    }
    Ok((visited, false))
}

fn part_one(filepath: &str) -> Result<usize> {
    let mut data = parse_data(filepath)?;
    let guard = find_guard_start(&mut data);
    let (visited, _) = simulate(&data, &guard, false)?;
    Ok(visited.len())
}

// part two functions

fn part_two(filepath: &str) -> Result<i32> {
    let mut data = parse_data(filepath)?;
    let guard = find_guard_start(&mut data);

    let (visited, _) = simulate(&data, &guard, false)?;
    let mut loop_count = 0;

    for &(vx, vy) in &visited {
        if (vx, vy) == (guard.x, guard.y) {
            continue;
        }

        if data[(vx, vy)] == '.' {
            data[(vx, vy)] = '#';
            let (_, loop_detected) = simulate(&data, &guard, true)?;
            if loop_detected {
                loop_count += 1;
            }
            data[(vx, vy)] = '.';
        }
    }

    Ok(loop_count)
}

fn main() {
    let filepath = "input.txt";

    println!("Hello, Advent of Code 2024!");
    
    let now = Instant::now();

    match part_one(filepath) {
        Ok(answer) => println!("Day 6, Part 1 answer: {}", answer),
        Err(e) => println!("Error: Could not calculate part one answer. {}", e),
    };

    println!("Day 6, Part 1 time elapsed {:.2?}", now.elapsed());
    let now = Instant::now();

    match part_two(filepath) {
        Ok(answer) => println!("Day 6, Part 2 answer: {}", answer),
        Err(e) => println!("Error: Could not calculate part two answer. {}", e),
    };

    println!("Day 6, Part 2 time elapsed {:.2?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let filepath = "part_one_test_input.txt";
        assert_eq!(part_one(filepath).unwrap(), 41);
    }

    #[test]
    fn test_part_two() {
        let filepath = "part_two_test_input.txt";
        assert_eq!(part_two(filepath).unwrap(), 6);
    }
}
