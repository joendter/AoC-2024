use std::fs;
use vector2d::Vector2D;

pub fn get_dimensions(lines: &Vec<Vec<u8>>) -> Vector2D<i64> {
    let xmax = lines[0].len().try_into().unwrap();
    let ymax = lines.len().try_into().unwrap();
    return Vector2D::new(xmax, ymax);
}

pub fn u8_at_position(lines: &Vec<Vec<u8>>, location: Vector2D<i64>) -> u8 {
    let y = TryInto::<usize>::try_into(location.y).unwrap();
    let x = TryInto::<usize>::try_into(location.x).unwrap();
    return lines[y][x];
}

pub fn is_in_bounds(v: &Vector2D<i64>, dimensions: &Vector2D<i64>) -> bool {
    return 0 <= v.x && v.x < dimensions.x && 0 <= v.y && v.y < dimensions.y;
}

pub fn change_u8_at_position(lines: &mut Vec<Vec<u8>>, location: Vector2D<i64>, new_value: u8) {
    let y = TryInto::<usize>::try_into(location.y).unwrap();
    let x = TryInto::<usize>::try_into(location.x).unwrap();
    lines[y][x] = new_value;
}

pub fn read_lines(file_path: String) -> Vec<String> {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines: Vec<String> = contents
        .split("\n")
        .filter(|&s| s != "")
        .map(|s| s.to_string())
        .collect();

    return lines;
}
