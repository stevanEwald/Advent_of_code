use std::collections::HashSet;
use rayon::prelude::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}
impl Direction {
    fn offsets(&self) -> (isize, isize) {
        match self {
            Self::North => (-1, 0),
            Self::East => (0, 1),
            Self::South => (1, 0),
            Self::West => (0, -1),
        }
    }
    fn turn_right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

const GUARD: char = '^';
const OBSTACLE: char = '#';

fn find_start_position(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for (y, row) in grid.iter().enumerate() {
        for (x, square) in row.iter().enumerate() {
            if *square == GUARD {
                return (y, x)
            }
        }
    }
    panic!("No start position found");
}

fn visited_squares(grid: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    let grid_size_y = grid.len();
    let grid_size_x = grid[0].len();

    let start_position = find_start_position(&grid);

    let mut current_direction = Direction::North;
    let (mut current_y, mut current_x) = start_position;

    let mut visited = HashSet::from([(current_y, current_x)]);

    loop {
        let (dy, dx) = current_direction.offsets();
        let Some(next_y) = current_y.checked_add_signed(dy) else { break };
        let Some(next_x) = current_x.checked_add_signed(dx) else { break };
        if next_y >= grid_size_y || next_x >= grid_size_x {
            break
        }
        let next_square = grid[next_y][next_x];

        if next_square == OBSTACLE {
            current_direction = current_direction.turn_right();
            continue
        }

        visited.insert((next_y, next_x));
        current_x = next_x;
        current_y = next_y;
    }
    return visited

}

pub fn part_1(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    return visited_squares(&grid).len() as i32
}

fn has_cycle(grid: &Vec<Vec<char>>, added_obsticale_position: (usize, usize)) -> bool {
    let grid_size_y = grid.len();
    let grid_size_x = grid[0].len();

    let start_position = find_start_position(&grid);

    let mut current_direction = Direction::North;
    let (mut current_y, mut current_x) = start_position;

    let mut visited = HashSet::from([(current_y, current_x, current_direction)]);

    loop {
        let (dy, dx) = current_direction.offsets();
        let Some(next_y) = current_y.checked_add_signed(dy) else { return false };
        let Some(next_x) = current_x.checked_add_signed(dx) else { return false };
        if next_y >= grid_size_y || next_x >= grid_size_x {
            return false
        }
        let next_square = grid[next_y][next_x];

        if next_square == OBSTACLE || (next_y, next_x) == added_obsticale_position {
            current_direction = current_direction.turn_right();
            continue
        }
        if visited.contains(&(next_y, next_x, current_direction)) {
            return true
        }
        visited.insert((next_y, next_x, current_direction));
        current_x = next_x;
        current_y = next_y;
    }
}

pub fn part_2(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut visited_squares = visited_squares(&grid);
    visited_squares.remove(&find_start_position(&grid));
    visited_squares.par_iter().filter(|&&square| has_cycle(&grid, square)).count() as i32
}