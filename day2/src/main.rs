use std::fs;

fn parseLine(line: &str) -> bool {
    let mut levels: Vec<i64> = Vec::new();
    for token in line.split(" ") {
        if token == "" {
            continue;
        }
        levels.push(token.parse::<i64>().unwrap());
    }
    for variant in removeOne(levels){
        if checkSafe(variant){
            return true;
        }
    }
    return false;
}

fn removeOne(levels: Vec<i64>) -> Vec<Vec<i64>>{
    let mut result = Vec::new();
    for i in 0..levels.len(){
        let mut copy = levels.clone();
        copy.remove(i);
        result.push(copy);
    }
    return result;
}

fn checkSafe(levels: Vec<i64>) -> bool {
    if (levels.len() < 2) {
        return false;
    }
    let direction = (levels[0] - levels[1]).signum();
    for i in 1..levels.len() {
        let diff = levels[i - 1] - levels[i];
        if diff * direction <= 0 {
            return false;
        }
        if diff.abs() > 3 {
            return false;
        }
    }
    return true;
}

fn main() {
    let file_path = "input";
    //println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines = contents.split("\n");
    let mut cnt = 0;
    for line in lines {
        if parseLine(line){
            cnt += 1;
            println!("{line} is safe");
        }
        else{
            println!("{line} is unsafe");
        }
    }
    println!("{cnt:?}");
}
