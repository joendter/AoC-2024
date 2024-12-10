use std::fs;
fn main() {
    let file_path = "input";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines: Vec<String> = contents
        .split("\n")
        .map(|s| s.to_string())
        .filter(|s| s != "")
        .collect();

    let line = lines[0].clone();

    let parsed = parseInput(&line);

    let comp = compacted(&parsed);

    let part1res = checksum(&comp);
    println!("{part1res}");

    let comp2 = alternative_compact(&parsed);
    println!("{comp2:?}");

    let part2 = checksum(&comp2);
    println!("{part2}");
}

fn parseInput(input: &String) -> Vec<i64> {
    let mut result = Vec::new();
    let mut empty = false;
    let mut counter = 0;

    for c in input.chars() {
        if !('0' <= c && c <= '9') {
            continue;
        }
        for count in 0..(c.to_string().parse::<i64>().unwrap()) {
            if empty {
                result.push(-1);
            } else {
                result.push(counter);
            }
        }

        if empty {
            counter += 1;
        }
        empty = !empty;
    }

    return result;
}

fn compacted(blocks: &Vec<i64>) -> Vec<i64> {
    let mut result = blocks.clone();
    let mut i = 0;
    let mut j = blocks.len() - 1;

    while i <= j {
        if (result[i] != -1) {
            i += 1;
            continue;
        }
        if (result[j] == -1) {
            j -= 1;
            continue;
        }
        result[i] = result[j];
        result[j] = -1;
    }
    return result;
}

fn checksum(blocks: &Vec<i64>) -> usize {
    let mut result: usize = 0;
    for i in 0..blocks.len() {
        if blocks[i] < 0 {
            continue;
        }
        result = result + i * TryInto::<usize>::try_into(blocks[i]).unwrap();
    }
    return result;
}

fn alternative_compact(blocks: &Vec<i64>) -> Vec<i64> {
    let mut result = blocks.clone();
    let mut j = blocks.len() - 1;
    let mut done = false;

    while j > 0 {
        while result[j] == -1 {
            j -= 1;
        }
        let current = result[j];
        let end = j;
        while j > 0 && result[j] == current {
            j -= 1;
        }
        let length = end - j;
        let mut new_location = 0;
        match find_empty_space(length, &result) {
            Some(i) => {
                new_location = i;
            }
            None => continue,
        }
        if new_location >= j {
            continue;
        }
        for k in 0..length {
            result[new_location + k] = current;
            result[j + 1 + k] = -1;
        }
        if current == 0 {
            done = true;
        }
    }

    return result;
}

fn find_empty_space(length: usize, blocks: &Vec<i64>) -> Option<usize> {
    for i in 0..(blocks.len() - length) {
        let mut valid = true;
        for j in 0..length {
            valid = valid && (blocks[i + j] == -1)
        }
        if valid {
            return Some(i);
        }
    }
    return None;
}
