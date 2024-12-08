use std::{
    fs::File,
    io::{BufRead, BufReader},
};
extern crate regex;
use regex::Regex;

pub fn dowork() {
    // regex: "mul(" + any amount of consecutive digits + "," + any amount more digits + ")"
    let re = Regex::new("mul[(][0-9]+,[0-9]+[)]").unwrap();
    let mul_vals = Regex::new("[0-9]+").unwrap();
    let mut sum = 0;

    for s in get_data() {
        let instr: Vec<&str> = re.find_iter(&s).map(|m| m.as_str()).collect();

        for mul in instr {
            let vals: Vec<i32> = mul_vals
                .find_iter(mul)
                .filter_map(|nums| nums.as_str().parse::<i32>().ok())
                .collect();

            sum+= vals[0] * vals[1];
        }
    }
    println!("sum is {sum}")
}

pub fn dowork2() {}

fn get_data() -> Vec<String> {
    let f = File::open("C://workspaces//aoc2024//src//3//src//input.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);

    let mut data: Vec<String> = Vec::new();
    for line in reader.lines() {
        data.push(match line {
            Ok(s) => s,
            Err(_s) => continue,
        });
    }
    return data;
}
