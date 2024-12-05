use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

fn main() {
    let file_path = "input";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();

    let spec = parseRawFile(&lines);

    let (conditions, orderings) = spec;
    let part1_result = part1(&orderings, &conditions);
    println!("pat1: {part1_result}");

    let part2_result = part2(&orderings, &conditions);
    println!("pat2: {part2_result}");
}

fn parseRawFile(lines: &Vec<String>) -> (Vec<(u64, u64)>, Vec<Vec<u64>>) {
    let mut raw_conditions: Vec<String> = Vec::new();
    let mut raw_orderings: Vec<String> = Vec::new();
    let mut state = false;
    for line in lines {
        if line == "" {
            state = true;
            continue;
        }
        if !state {
            raw_conditions.push(line.to_string());
        } else {
            raw_orderings.push(line.to_string())
        }
    }

    let conditions = parseConditions(&raw_conditions);
    let orderings = parseOrderings(&raw_orderings);

    return (conditions, orderings);
}

fn parseCondition(line: &String) -> (u64, u64) {
    let parsed: Vec<u64> = line.split("|").map(|t| t.parse::<u64>().unwrap()).collect();
    return (parsed[0], parsed[1]);
}

fn parseOrdering(line: &String) -> Vec<u64> {
    let parsed: Vec<u64> = line.split(",").map(|t| t.parse::<u64>().unwrap()).collect();
    return parsed;
}

fn parseConditions(lines: &Vec<String>) -> Vec<(u64, u64)> {
    return lines.into_iter().map(|l| parseCondition(l)).collect();
}

fn parseOrderings(lines: &Vec<String>) -> Vec<Vec<u64>> {
    return lines.into_iter().map(|l| parseOrdering(l)).collect();
}

fn isOrderingValid(ordering: &Vec<u64>, conditions: &Vec<(u64, u64)>) -> bool {
    return conditions
        .iter()
        .map(|condition| isConditionValid(&ordering, condition))
        .fold(true, |acc, x| acc && x);
}

fn isConditionValid(ordering: &Vec<u64>, condition: &(u64, u64)) -> bool {
    let (first, second) = condition;
    let first_index = ordering.iter().position(|x| x == first);
    let second_index = ordering.iter().position(|x| x == second);
    if first_index.is_none() || second_index.is_none() {
        return true;
    }
    return first_index.unwrap() <= second_index.unwrap();
}

fn getMidpoint(ordering: &Vec<u64>) -> u64 {
    let mid = (ordering.len() - 1) / 2;
    return ordering[mid];
}

fn part1(orderings: &Vec<Vec<u64>>, conditions: &Vec<(u64, u64)>) -> u64 {
    let mut accumulator = 0;
    for ordering in orderings {
        if isOrderingValid(ordering, conditions) {
            accumulator += getMidpoint(ordering);
        }
    }
    return accumulator;
}

fn relevantConditions(ordering: &Vec<u64>, conditions: &Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    return conditions
        .iter()
        .filter(|(a, _)| ordering.contains(a))
        .map(|&a| a)
        .collect();
}

fn get_dependent(conditions: &Vec<(u64, u64)>) -> Vec<u64> {
    return conditions.iter().map(|&(_, a)| a).unique().collect();
}
fn get_independent(conditions: &Vec<(u64, u64)>) -> Vec<u64> {
    let dependents = get_dependent(conditions);
    return conditions
        .iter()
        .filter(|(a, _)| dependents.contains(a))
        .map(|&(a, _)| a)
        .collect();
}

fn get_toposort(ordering: &Vec<u64>, conditions: &Vec<(u64, u64)>) -> Vec<u64> {
    let mut left = ordering.clone();
    let mut result = Vec::new();
    let mut conditions_left = relevantConditions(&left, conditions);

    while left.len() > 0 {
        for page in &left {
            if !get_dependent(&conditions_left).contains(&page) {
                result.push(*page);
            }
        }
        left = left
            .iter()
            .filter(|&a| !result.contains(a))
            .map(|&a| a)
            .collect();
        conditions_left = relevantConditions(&left, conditions);
    }
    return result;
}
fn part2(orderings: &Vec<Vec<u64>>, conditions: &Vec<(u64, u64)>) -> u64 {
    let mut accumulator = 0;
    for ordering in orderings {
        if isOrderingValid(ordering, conditions) {
            continue;
        }
        accumulator += getMidpoint(&get_toposort(ordering, conditions));
    }
    return accumulator;
}
