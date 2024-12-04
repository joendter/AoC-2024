use std::fs;

fn main() {
    let file_path = "input";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut lines: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
    lines.retain(|s| s != "");
    let vars = variations(lines);
    println!("{vars:?}");
    let mut count = 0;
    for variant in vars {
        for line in variant {
            count += checkline(line);
        }
    }

    println!("found {count}");
}

fn variations(lines: Vec<String>) -> Vec<Vec<String>> {
    let mut result: Vec<Vec<String>> = Vec::new();

    result.push(lines.clone());
    result.push(reversed(&lines));
    let sideways = rotated(&lines);
    result.push(reversed(&sideways));
    let diag1 = diagonals(&lines);
    let diag2 = diagonals(&reversed(&sideways));
    result.push(reversed(&diag1));
    result.push(reversed(&diag2));
    result.push(diag1);
    result.push(diag2);
    result.push(sideways);

    return result;
}

fn reversed(lines: &Vec<String>) -> Vec<String> {
    return lines
        .into_iter()
        .map(|l| l.chars().rev().collect::<String>())
        .collect();
}

fn char_at_position(lines: &Vec<String>, x: usize, y: usize) -> char {
    return lines[y].as_bytes()[x] as char;
}

fn rotated(lines: &Vec<String>) -> Vec<String> {
    let xmax = lines[0].len();
    let ymax = lines.len();
    let mut result: Vec<String> = Vec::new();

    for new_y in 0..xmax {
        let mut tmp: String = "".to_string();
        for new_x in 0..ymax {
            tmp.push(char_at_position(lines, new_y, new_x));
        }
        result.push(tmp);
    }
    return result;
}

fn diagonals(lines: &Vec<String>) -> Vec<String> {
    let xmax = lines[0].len();
    let ymax = lines.len();
    let mut result: Vec<String> = Vec::new();

    for sum in 0..(xmax + ymax - 1) {
        let mut tmp: String = "".to_string();
        for y in 0..(sum + 1) {
            let x = sum - y;
            if (x >= xmax || y >= ymax) {
                continue;
            };
            tmp.push(char_at_position(lines, x, y));
        }
        result.push(tmp);
    }

    return result;
}

fn checkline(line: String) -> usize {
    return line.matches("XMAS").count();
}
