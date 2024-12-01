use std::env;
use std::fs;

fn main(){
    let file_path = "input";
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines = contents.split("\n");
    let mut ids1 : Vec<i64> = Vec::new();
    let mut ids2 : Vec<i64> = Vec::new();
    let mut state: bool = false;
    for line in lines {
        for token in line.split(" "){
            if (token == ""){
continue;
            }
            let my_int = token.parse::<i64>().unwrap();
            if (!state){
                ids1.push(my_int);
                state = !state;
            }
            else {
                ids2.push(my_int);
                state = !state;
            }
        }
    }
    println!("{ids1:?}\n\n{ids2:?}");
    ids1.sort();
    ids2.sort();
    println!("{ids1:?}\n\n{ids2:?}");

    let mut accu : i64 = 0;
    for i in 0..(ids1.len()){
        accu += (ids1[i] - ids2[i]).abs();
    }
    println!("{accu:?}");

    let mut accu2 : i64 = 0;
    for a in &ids1{
        for b in &ids2{
            if a == b {
                accu2 += a;
            }
        }
    }
    println!("{accu2:?}");
}
