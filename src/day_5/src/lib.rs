use std::{
    collections::{HashMap, HashSet}, convert::TryInto, fs::File, io::{BufRead, BufReader}
};

pub fn dowork() {
    let temp = get_data();
    let rules = temp.0;
    let inputs = temp.1;

    let mut sum = 0;
    let mut count = 0;
    for input in inputs {
        count += 1;
        if all_rules_pass(&rules, &input) {
            sum += input[input.len() / 2]
        }
    }
    println!("sum = {sum}")
}

pub fn dowork2() {
    let temp = get_data();
    let rules = temp.0;
    let inputs = temp.1;

    let mut sum = 0;
    let mut count = 0;
    for input in inputs {
        count += 1;
        if !all_rules_pass(&rules, &input) {
            let mut to_fix = vec![0; input.len()];
            to_fix.clone_from_slice(&input);

            fix(to_fix, &rules);

            sum += input[input.len() / 2]
        }
    }
    println!("sum = {sum}")
}

fn all_rules_pass(rules: &HashMap<i32, HashSet<i32>>, input: &Vec<i32>) -> bool {
    let mut prior = HashSet::<i32>::new();
    for val in input {
        // if we have rules for this num, check them
        if rules.contains_key(val) {
            let the_rules = &rules[val];
            for rule in the_rules {
                if prior.contains(rule) {
                    return false;
                }
            }
        } else {
            continue;
        }
        prior.insert(*val);
    }
    return true;
}

fn value_passes(rules: &HashMap<i32, HashSet<i32>>, input: &Vec<i32>, index: usize) -> bool {
    let mut next = HashSet::<i32>::new();
    for next_val in index + 1..input.len()-1{
        next.insert(input[next_val]);
    }

    let val = input[index];
    // if we have rules for this num, check them
    if rules.contains_key(&val) {
        let the_rules = &rules[&val];
        for rule in the_rules {
            if next.contains(rule) {
                return false;
            }
        }
    } 

    return true;
}

fn fix(mut to_fix: Vec<i32>, rules: &HashMap<i32, HashSet<i32>>) {
    if all_rules_pass(rules, &to_fix) {
        return;
    }

    let mut index: usize = 0;
    let mut fixed = false;
    loop {
        if index == to_fix.len(){
            return;
        }
        
        // nothing to do with a good value
        if value_passes(rules, &to_fix, index){
            index +=1;
            continue;
        }
        
        let new_index = match get_correct_index(index, &to_fix, rules) {
            Some(index) => index,

            // if we don't find anything to swap to, that means that the index was correct
            None => {
                index += 1;
                continue;
            }
        };

        to_fix.swap(index, new_index);

        fixed = all_rules_pass(rules, &to_fix);

        if fixed {
            break;
        }
        index += 1;
    }
}

fn get_correct_index(
    index_in: usize,
    to_fix: &Vec<i32>,
    rules: &HashMap<i32, HashSet<i32>>,
) -> Option<usize> {
    let val = to_fix[index_in];
    let rules_for_val = &rules[&val];

    for after in index_in + 1..to_fix.len() {
        if rules_for_val.contains(&to_fix[after]) {
            return Some(after);
        } else {
            continue;
        }
    }
    return None;
}

fn get_data() -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let f = File::open("C:/workspaces/aoc2024/src/day_5/src/input.txt").unwrap();
    let reader = BufReader::new(f);

    let mut rules_done = false;
    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut inputs: Vec<Vec<i32>> = Vec::new();
    for line in reader.lines() {
        if line.is_err() {
            continue;
        }

        let raw = line.unwrap();

        if raw == "" {
            rules_done = true;
            continue;
        }

        if !rules_done {
            let vals: Vec<String> = raw.split("|").map(|s| s.to_string()).collect();

            let num1 = vals[0].parse::<i32>().unwrap();
            let num2 = vals[1].parse::<i32>().unwrap();

            if rules.contains_key(&num1) {
                match rules.get_mut(&num1) {
                    Some(l) => l.insert(num2),
                    None => continue,
                };
            } else {
                let mut new_set = HashSet::<i32>::new();
                new_set.insert(num2);
                rules.insert(num1, new_set);
            }
        } else {
            inputs.push(raw.split(",").map(|s| s.parse::<i32>().unwrap()).collect());
        }
    }

    return (rules, inputs);
}
