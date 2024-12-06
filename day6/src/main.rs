use std::collections::HashSet;
use std::fs;

fn main() {
    let file_path = "CS-input";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines: Vec<String> = contents
        .split("\n")
        .map(|s| s.to_string())
        .filter(|s| s != "")
        .collect();

    let (locs, _) = find_guard_locations(&lines);

    println!("{locs:?}");
    let part1 = locs.len();
    println!("{part1:?}");

    let mut positions = HashSet::new();
    for vari in variations(&lines) {
        let (_, circle) = find_guard_locations(&vari.0);
        if circle {
            positions.insert(vari.1);
        }
    }

    let counter = positions.len();

    println!("{counter:?}");

    visualise(get_dimensions(&lines), positions);
}

#[derive(Hash, Eq, PartialEq, Clone)]
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

fn get_dimensions(lines: &Vec<String>) -> (usize, usize){
    let xmax = lines[0].len();
    let ymax = lines.len();
    return (xmax, ymax);
}

fn char_at_position(lines: &Vec<String>, x: usize, y: usize) -> char {
    return lines[y - 1].as_bytes()[x - 1] as char;
}

fn get_initial_guard_location(lines: &Vec<String>) -> (usize, usize) {
    let xmax = lines[0].len();
    let ymax = lines.len();
    //println!("{xmax},{ymax}");
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

fn find_guard_locations(lines: &Vec<String>) -> (HashSet<(usize, usize)>, bool) {
    let mut states = HashSet::new();

    let mut guard_location = get_initial_guard_location(lines);
    let mut guard_direction = Direction::Up;

    while is_in_bounds(lines, &guard_location)
        && !states.contains(&(guard_location, guard_direction.clone()))
    {
        states.insert((guard_location, guard_direction.clone()));
        (guard_location, guard_direction) =
            get_next_guard_state(&guard_location, guard_direction, lines);
    }

    let result = states.iter().map(|&(a, _)| a).collect();

    return (
        result,
        states.contains(&(guard_location, guard_direction.clone())),
    );
}

fn variations(lines: &Vec<String>) -> Vec<(Vec<String>, (usize,usize))> {
    let xmax = lines[0].len();
    let ymax = lines.len();
    let mut result = Vec::new();
    for x in 1..xmax + 1 {
        for y in 1..ymax + 1 {
            if char_at_position(lines, x, y) == '^' {
                continue;
            }
            result.push((variation(lines, (x, y)), (x,y)));
        }
    }
    return result;
}

fn variation(lines: &Vec<String>, position: (usize, usize)) -> Vec<String> {
    let xmax = lines[0].len();
    let ymax = lines.len();
    let mut result: Vec<String> = Vec::new();

    for y in 1..ymax + 1 {
        let mut tmp: String = "".to_string();
        for x in 1..xmax + 1 {
            if (x, y) != position {
                tmp.push(char_at_position(lines, x, y));
            } else {
                tmp.push('#');
            }
        }
        result.push(tmp);
    }
    return result;
}

fn visualise(dimensions: (usize, usize), positions: HashSet<(usize,usize)>) {
    for y in 0..dimensions.1{
        let mut line = "".to_string();
        for x in 0..dimensions.0{
            if positions.contains(&(x+1,y+1)){
                line.push('O');
            }
            else{line.push('.')};
        }
        println!("{line}");
    }
}
