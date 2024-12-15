use std::fs;
use vector2d::Vector2D;
use AoClib::*;

const boxu8: u8 = 79;
const robotu8: u8 = 64;
const wallu8: u8 = 35;
const freeu8: u8 = 46;

fn main() {
    let file_path = "input";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let split: Vec<String> = contents.split("\n\n").map(|s| s.to_string()).collect();
    let rawWarehouse = &split[0];
    let rawMoves = &split[1];

    let warehouse: Vec<Vec<u8>> = rawWarehouse
        .split("\n")
        .filter(|&s| s != "")
        .map(|s| s.as_bytes().to_owned().iter().map(|&c| c).collect())
        .collect();

    println!("{warehouse:?}");

    let directions = parse_directions(rawMoves);
    println!("{directions:?}");

    let mut state = warehouse.clone();
    for d in directions {
        state = next_state(&state, d);
    }
    println!("{state:?}");

    let res = score(&state);
    println!("{res:?}");
}

fn next_state(warehouse: &Vec<Vec<u8>>, direction: Vector2D<i64>) -> Vec<Vec<u8>> {
    let mut result = warehouse.clone();
    let initial_position = find_robot_position(&warehouse);
    let mut position = initial_position + direction;
    while u8_at_position(warehouse, position) == boxu8 {
        position = position + direction;
    }
    if u8_at_position(warehouse, position) == wallu8 {
        return result;
    }

    let pos1 = initial_position + direction;
    change_u8_at_position(&mut result, initial_position, freeu8);
    change_u8_at_position(&mut result, position, boxu8);
    change_u8_at_position(&mut result, pos1, robotu8);

    return result;
}

fn find_robot_position(warehouse: &Vec<Vec<u8>>) -> Vector2D<i64> {
    let dims = get_dimensions(&warehouse);
    for x in 0..dims.x {
        for y in 0..dims.y {
            if u8_at_position(&warehouse, Vector2D::new(x, y)) == robotu8 {
                return Vector2D::new(x, y);
            }
        }
    }
    panic!("expected robot to exist");
}

fn parse_directions(directions: &String) -> Vec<Vector2D<i64>> {
    return directions
        .chars()
        .map(|c| match c {
            '<' => Vector2D::new(-1, 0),
            '>' => Vector2D::new(1, 0),
            '^' => Vector2D::new(0, -1),
            'v' => Vector2D::new(0, 1),
            _ => Vector2D::new(0, 0),
        })
        .filter(|&v| v != Vector2D::new(0, 0))
        .collect();
}

fn score(warehouse: &Vec<Vec<u8>>) -> i64 {
    let mut result = 0;
    let dims = get_dimensions(&warehouse);
    for x in 0..dims.x {
        for y in 0..dims.y {
            if u8_at_position(&warehouse, Vector2D::new(x, y)) == boxu8 {
                result = result + x + 100 * y
            }
        }
    }
    return result;
}
