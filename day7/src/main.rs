use evalexpr::*;
use std::fs;

fn main() {
    let file_path = "input";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut lines: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
    lines.retain(|s| s != "");

    let test_term = "1+2+3*4|5".to_string();
    let b = evalu(&test_term);

    println!("test: {b}");

    let a: u64 = lines.iter().map(|l| check_line(l)).sum::<u64>();

    println!("part1: {a}");
}

fn evalu(expression: &String) -> u64 {
    let mut operand1 = "0".to_string();
    let mut operand2 = "".to_string();
    let mut operator = '+';
    for c in expression.chars() {
        if ['*', '+', '|'].contains(&c) {
            if operator == '|' {
                operand1 = format!("{operand1}{operand2}");
            } else {
                let e = format!("{operand1}{operator}{operand2}");
                operand1 = eval(&e).unwrap().as_int().unwrap().to_string();
            }
            operand2 = "".to_string();
            operator = c;
            continue;
        }
        if "0123456789".chars().collect::<Vec<char>>().contains(&c) {
            operand2.push(c);
            continue;
        }
        panic!();
    }
    if operator == '|' {
        operand1 = format!("{operand1}{operand2}");
    } else {
        let e = format!("{operand1}{operator}{operand2}");
        operand1 = eval(&e).unwrap().as_int().unwrap().to_string();
    }
    return operand1.parse().unwrap();
}

fn equation_variants(equation: &String) -> Vec<String> {
    let mut result = Vec::new();
    let number_of_operators = get_number_of_operators(equation);
    let max_variant = u64::pow(3, number_of_operators);

    for i in 0..max_variant {
        result.push(equation_variant(equation, i));
    }

    return result;
}

fn equation_variant(equation: &String, index: u64) -> String {
    let mut result = "".to_string();
    let mut n_operator = 0;
    for c in equation.chars() {
        if c != ' ' {
            result.push(c);
            continue;
        }
        let remainder = (index / u64::pow(3, n_operator)) % 3;
        match remainder {
            0 => result.push('+'),
            1 => result.push('*'),
            2 => result.push('|'),
            2_u64..=u64::MAX => panic!(),
        }
        n_operator += 1;
    }

    //println!("{result}");

    return result;
}

fn get_number_of_operators(equation: &String) -> u32 {
    return equation
        .chars()
        .filter(|&c| c == ' ')
        .collect::<Vec<char>>()
        .len()
        .try_into()
        .unwrap();
}

// 0 if not doable, n if doable
fn check_line(line: &String) -> u64 {
    let tmp: Vec<String> = line.split(": ").map(|s| s.to_string()).collect();
    let result = tmp[0].parse::<u64>().unwrap();
    let equation = tmp[1].clone();

    let valid_equations = equation_variants(&equation)
        .into_iter()
        .filter(|e| result == evalu(e))
        .collect::<Vec<String>>();
    println!("{result} : {valid_equations:?}");

    if valid_equations.len() == 0 {
        return 0;
    }

    return result;
}
