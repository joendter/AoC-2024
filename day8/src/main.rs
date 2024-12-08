use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use vector2d::Vector2D;

fn main() {
    let file_path = "input";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut lines: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
    lines.retain(|s| s != "");

    let antinodes = get_all_antinodes(&lines);

    //println!("{antinodes:?}");
    visualise(get_dimensions(&lines), &antinodes);

    let part1 = antinodes.len();
    println!("part1: {part1}");
}

fn get_dimensions(lines: &Vec<String>) -> Vector2D<i64> {
    let xmax = lines[0].len().try_into().unwrap();
    let ymax = lines.len().try_into().unwrap();
    return Vector2D::new(xmax, ymax);
}

fn char_at_position(lines: &Vec<String>, location: Vector2D<i64>) -> char {
    let y = TryInto::<usize>::try_into(location.y).unwrap();
    let x = TryInto::<usize>::try_into(location.x).unwrap();
    return lines[y].as_bytes()[x] as char;
}

fn is_in_bounds(v: &Vector2D<i64>, dimensions: &Vector2D<i64>) -> bool {
    return 0 <= v.x && v.x < dimensions.x && 0 <= v.y && v.y < dimensions.y;
}

fn get_antinodes(
    node1: Vector2D<i64>,
    node2: Vector2D<i64>,
    dimensions: &Vector2D<i64>,
) -> HashSet<Vector2D<i64>> {
    let mut result = HashSet::new();
    if node1 != node2 {
        let diff = atomic(&(node1 - node2));
        let mut pos = node1;
        while is_in_bounds(&pos, &dimensions) {
            result.insert(pos);
            pos = pos + diff;
        }
        pos = node2;
        while is_in_bounds(&pos, &dimensions) {
            result.insert(pos);
            pos = pos - diff;
        }
    }
    return result;
}

fn get_antennae(lines: &Vec<String>) -> HashMap<char, HashSet<Vector2D<i64>>> {
    let mut result: HashMap<char, HashSet<Vector2D<i64>>> = HashMap::new();

    let dimensions = get_dimensions(&lines);

    for x in 0..dimensions.x {
        for y in 0..dimensions.y {
            let position = Vector2D::new(x, y);
            let c = char_at_position(lines, position);
            if c == '.' {
                continue;
            }
            match result.get_mut(&c) {
                Some(_) => {}
                None => {
                    result.insert(c, HashSet::new());
                }
            }
            match result.get_mut(&c) {
                Some(set) => {
                    set.insert(position);
                }
                None => {}
            }
        }
    }

    return result;
}

fn get_all_antinodes(lines: &Vec<String>) -> HashSet<Vector2D<i64>> {
    let antennae = get_antennae(lines);
    let mut result = HashSet::new();
    let dimensions = get_dimensions(lines);
    for anten in antennae.values() {
        for antenna1 in anten {
            for antenna2 in anten {
                result = result
                    .union(&get_antinodes(*antenna1, *antenna2, &dimensions))
                    .map(|&a| a)
                    .collect();
            }
        }
    }

    result = result
        .iter()
        .filter(|v| is_in_bounds(v, &dimensions))
        .map(|&a| a)
        .collect();

    return result;
}

fn visualise(dimensions: Vector2D<i64>, positions: &HashSet<Vector2D<i64>>) {
    for y in 0..dimensions.y {
        let mut line = "".to_string();
        for x in 0..dimensions.x {
            if positions.contains(&Vector2D::new(x, y)) {
                line.push('O');
            } else {
                line.push('.')
            };
        }
        println!("{line}");
    }
}

fn atomic(vec: &Vector2D<i64>) -> Vector2D<i64> {
    for dd in 0..vec.x {
        let d = vec.x - dd;
        let shortened_vector = Vector2D::new(vec.x / d, vec.y / d);
        if shortened_vector * d == *vec {
            return shortened_vector.clone();
        }
    }
    return *vec;
}
