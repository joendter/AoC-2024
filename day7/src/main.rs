use evalexpr::*;
use std::collections::HashMap;
use std::fs;

fn main() {
    let file_path = "example";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut lines: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
    lines.retain(|s| s != "");

    let a: u64 = lines.iter().map(|l| check_line(l)).sum::<u64>();

    println!("part1: {a}");
}

/*
fn eval(expression: &String) -> u64 {
    return 0;
}
*/

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
        if remainder <= 1 {
            result = format!("({result})");
            match remainder {
                0 => result.push('+'),
                1 => result.push('*'),
                2_u64..=u64::MAX => panic!(),
            }
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
    let mut context = HashMap::new();

    context.insert(
        "concatenate".to_string(),
        evalexpr::Function::<DefaultNumericTypes>::new(|a| concatenate(a)),
    );

    let valid_equations = equation_variants(&equation)
        .into_iter()
        .filter(|e| result == eval(e).unwrap().as_int().unwrap().try_into().unwrap())
        .collect::<Vec<String>>();
    println!("{result} : {valid_equations:?}");

    if valid_equations.len() == 0 {
        return 0;
    }

    return result;
}

fn concatenate(args: Value) -> Result<Value, EvalexprError> {
    // Ensure there's exactly one argument
    if args.len() != 2 {
        return Err(
            evalexpr::EvalexprError::<DefaultNumericTypes>::wrong_function_argument_amount(
                args.len(),
                2,
            ),
        );
    }

    // Make sure the argument is a number
    match &args[0] {
        Value::Int(i) => match &args[1] {
            Value::Int(j) => {
                let exponent = j.ilog10() + 1;
                let result = i * 10_i64.pow(exponent) + j;
                Ok(Value::Int(result))
            }
            _ => Err(evalexpr::EvalexprError::<DefaultNumericTypes>::type_error(
                Value::Float(42.0),
                Vec::new(),
            )),
        },
        _ => Err(evalexpr::EvalexprError::<DefaultNumericTypes>::type_error(
            Value::Float(42.0),
            Vec::new(),
        )),
    }
}
