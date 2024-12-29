use std::{collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader}};

pub fn dowork(){
    let temp = get_data();
    let rules = temp.0;
    let inputs = temp.1;

    let mut sum = 0;
    let mut count = 0;
    for input in inputs{
        count +=1;
        if all_rules_pass(&rules, &input){
            sum+= input[input.len()/2]
        }
    }
    println!("sum = {sum}")
}

pub fn dowork2(){

}

fn all_rules_pass(rules: &HashMap<i32, Vec<i32>>, input: &Vec<i32>) -> bool{
    let mut prior = HashSet::<i32>::new();
    for val in input{
        // if we have rules for this num, check them
        if rules.contains_key(val){
            let the_rules = &rules[val];
            for rule in the_rules{
                if prior.contains(rule){
                    return false;
                }
            }
        }
        else{
            continue;
        }
        prior.insert(*val);
    }
    return true;
}

fn get_data() -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let f = File::open("C:/workspaces/aoc2024/src/day_5/src/input.txt").unwrap();
    let reader = BufReader::new(f);

    let mut rulesDone = false;
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut inputs : Vec<Vec<i32>> = Vec::new();
    for line in reader.lines() {
        if line.is_err() {
            continue;
        }

        let raw = line.unwrap();

        if raw == "" {
            rulesDone = true;
            continue;
        }

        if !rulesDone {
            let vals: Vec<String> = raw.split("|").map(|s| s.to_string()).collect();

            let num1 = vals[0].parse::<i32>().unwrap();
            let num2 = vals[1].parse::<i32>().unwrap();

            if rules.contains_key(&num1) {
                match rules.get_mut(&num1) {
                    Some(l) => l.push(num2),
                    None => continue,
                };
            } else {
                let mut new_vec = Vec::<i32>::new();
                new_vec.push(num2);
                rules.insert(num1, new_vec);
            }
        }
        else {
            inputs.push(raw.split(",").map(|s| s.parse::<i32>().unwrap()).collect());
        }
    }

    return (rules, inputs);
}
