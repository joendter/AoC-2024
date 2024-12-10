use std::collections::HashSet;
use std::fs;
use vector2d::Vector2D;

fn main() {
    let file_path = "input";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines: Vec<Vec<u8>> = contents
        .split("\n")
        .filter(|&s| s != "")
        .map(|s| s.as_bytes().to_owned().iter().map(|&c| c - 0x30).collect())
        .collect();

    println!("{:?}", find_end_positions(&lines, &Vector2D::new(2, 0)));

    let part1: usize = find_initial_positions(&lines)
        .iter()
        .map(|p| find_end_positions(&lines, p).len())
        .sum();
    println!("{part1}");

    let part2: usize = find_initial_positions(&lines)
        .iter()
        .map(|p| find_end_positions2(&lines, p).len())
        .sum();
    println!("{part2}");
}

fn valid_next_steps(lines: &Vec<Vec<u8>>, position: Vector2D<i64>) -> HashSet<Vector2D<i64>> {
    let mut result = HashSet::new();
    let bounds = get_dimensions(lines);

    for delta in [
        Vector2D::new(0, 1),
        Vector2D::new(1, 0),
        Vector2D::new(0, -1),
        Vector2D::new(-1, 0),
    ] {
        let new_position = position + delta;
        if !is_in_bounds(&new_position, &bounds) {
            continue;
        }
        if u8_at_position(lines, new_position.clone()) - u8_at_position(lines, position) != 1 {
            continue;
        }
        result.insert(new_position);
    }

    return result;
}

fn get_new_positions(
    lines: &Vec<Vec<u8>>,
    positions: HashSet<Vector2D<i64>>,
) -> HashSet<Vector2D<i64>> {
    return positions
        .iter()
        .map(|&p| valid_next_steps(lines, p))
        .fold(HashSet::new(), |acc, e| acc.union(&e).map(|&a| a).collect());
}

fn find_end_positions(
    lines: &Vec<Vec<u8>>,
    initial_position: &Vector2D<i64>,
) -> HashSet<Vector2D<i64>> {
    let mut positions = HashSet::new();
    positions.insert(*initial_position);
    for _ in 0..9 {
        positions = get_new_positions(lines, positions);
    }
    return positions;
}

fn find_initial_positions(lines: &Vec<Vec<u8>>) -> HashSet<Vector2D<i64>> {
    let dimensions = get_dimensions(lines);
    let mut result = HashSet::new();
    for x in 0..dimensions.x {
        for y in 0..dimensions.y {
            let position = Vector2D::new(x, y);
            if u8_at_position(lines, position) == 0 {
                result.insert(position);
            }
        }
    }
    return result;
}

fn get_dimensions(lines: &Vec<Vec<u8>>) -> Vector2D<i64> {
    let xmax = lines[0].len().try_into().unwrap();
    let ymax = lines.len().try_into().unwrap();
    return Vector2D::new(xmax, ymax);
}

fn u8_at_position(lines: &Vec<Vec<u8>>, location: Vector2D<i64>) -> u8 {
    let y = TryInto::<usize>::try_into(location.y).unwrap();
    let x = TryInto::<usize>::try_into(location.x).unwrap();
    return lines[y][x];
}

fn is_in_bounds(v: &Vector2D<i64>, dimensions: &Vector2D<i64>) -> bool {
    return 0 <= v.x && v.x < dimensions.x && 0 <= v.y && v.y < dimensions.y;
}
fn get_new_positions2(lines: &Vec<Vec<u8>>, positions: Vec<Vector2D<i64>>) -> Vec<Vector2D<i64>> {
    return positions
        .iter()
        .map(|&p| valid_next_steps(lines, p))
        .fold(Vec::new(), |acc, e| {
            let mut result = acc.clone();
            for p in e {
                result.push(p);
            }
            return result;
        });
}

fn find_end_positions2(
    lines: &Vec<Vec<u8>>,
    initial_position: &Vector2D<i64>,
) -> Vec<Vector2D<i64>> {
    let mut positions = Vec::new();
    positions.push(*initial_position);
    for _ in 0..9 {
        positions = get_new_positions2(lines, positions);
    }
    return positions;
}
