use std::collections::HashSet;
use std::fs;

fn main() {
    let file_path = "input";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines: Vec<String> = contents
        .split("\n")
        .map(|s| s.to_string())
        .filter(|s| s != "")
        .collect();

    let locs = find_guard_locations(&lines);

    println!("{locs:?}");
    let part1 = locs.len();
    println!("{part1:?}");
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn rotated(&self) -> Direction {
        match self {
            Direction::Up => return Direction::Right,
            Direction::Right => return Direction::Down,
            Direction::Down => return Direction::Left,
            Direction::Left => return Direction::Up,
        }
    }
    fn next_position(&self, position: &(usize, usize)) -> (usize, usize) {
        match self {
            Direction::Up => return (position.0 + 0, position.1 - 1),
            Direction::Right => return (position.0 + 1, position.1 + 0),
            Direction::Down => return (position.0 + 0, position.1 + 1),
            Direction::Left => return (position.0 - 1, position.1 + 0),
        }
    }
    fn rotate(&mut self) {
        *self = self.rotated();
    }
}

fn char_at_position(lines: &Vec<String>, x: usize, y: usize) -> char {
    return lines[y - 1].as_bytes()[x - 1] as char;
}

fn get_initial_guard_location(lines: &Vec<String>) -> (usize, usize) {
    let xmax = lines[0].len();
    let ymax = lines.len();
    println!("{xmax},{ymax}");
    for x in 1..xmax + 1 {
        for y in 1..ymax + 1 {
            if char_at_position(lines, x, y) == '^' {
                return (x, y);
            }
        }
    }
    panic!("expected to find the guard somewhere");
}

fn is_in_bounds(lines: &Vec<String>, position: &(usize, usize)) -> bool {
    let xmax = lines[0].len();
    let ymax = lines.len();
    return 1 <= position.0 && position.0 <= xmax && 1 <= position.1 && position.1 <= ymax;
}

fn collides(position: (usize, usize), lines: &Vec<String>) -> bool {
    if !is_in_bounds(lines, &position) {
        return false;
    };
    return char_at_position(lines, position.0, position.1) == '#';
}

fn get_next_guard_state(
    guard_location: &(usize, usize),
    guard_direction: Direction,
    lines: &Vec<String>,
) -> ((usize, usize), Direction) {
    let mut new_direction = guard_direction;
    while collides(new_direction.next_position(guard_location), &lines) {
        new_direction.rotate();
    }
    let new_location = new_direction.next_position(guard_location);

    return (new_location, new_direction);
}

fn find_guard_locations(lines: &Vec<String>) -> HashSet<(usize, usize)> {
    let mut result = HashSet::new();

    let mut guard_location = get_initial_guard_location(lines);
    let mut guard_direction = Direction::Up;

    while is_in_bounds(lines, &guard_location) {
        result.insert(guard_location);
        (guard_location, guard_direction) =
            get_next_guard_state(&guard_location, guard_direction, lines);
    }

    return result;
}
