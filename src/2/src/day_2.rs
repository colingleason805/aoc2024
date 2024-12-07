use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn dowork() {
    let mut l1: Vec<Vec<i32>> = Vec::new();
    populate_lists(&mut l1);

    let mut safe_count = 0;
    for l2 in l1 {
        if is_safe(&l2) {
            safe_count += 1
        }
    }
    println!("Safe count is {safe_count}")
}

pub fn dowork2() {
    let mut l1: Vec<Vec<i32>> = Vec::new();
    populate_lists(&mut l1);

    let mut safe_count = 0;
    for l2 in l1 {
        let len = l2.len();
        if is_safe(&l2){
            safe_count+=1;
        }
        else {
            let mut can_dampen = false;
            for i in 0..len {
                let mut dampened = l2.clone();
                dampened.remove(i);
                if is_safe(&dampened) {
                    can_dampen = true;
                    break;
                }
            }
            if can_dampen {
                safe_count += 1
            }
        }
    }
    println!("Dampened safe count is {safe_count}")
}

fn is_safe(list: &Vec<i32>) -> bool{
    let mut safe = false;
    let mut decreasing: bool = false;
    let mut last: i32 = -1;
    let mut i = 0;
    let len = list.len();
    for val in list {
        if i == 0 {
            last = *val;
            i += 1;
            continue;
        } else if i == 1 {
            decreasing = last > *val;
        }

        let diff = val - last;
        if diff.abs() > 3 || diff.abs() < 1 {
            break;
        } else if decreasing && diff.is_positive() {
            break;
        } else if !decreasing && diff.is_negative() {
            break;
        }

        //we got through all entries in the list. its safe
        if i == len - 1 {
            safe = true;
            break;
        }

        last = *val;
        i += 1;
    }
    return safe;
}

fn populate_lists(l1: &mut Vec<Vec<i32>>) {
    let f = File::open("C://workspaces//aoc2024//src//2//src//input.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);

    for line in reader.lines() {
        let prs = line.unwrap();
        let vals: Vec<i32> = prs
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        l1.push(vals);
    }
}
