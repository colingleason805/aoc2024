use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn dowork() {
    let mut sum: i32 = 0;

    let mut l1: Vec<i32> = Vec::new();
    let mut l2: Vec<i32> = Vec::new();

    populate_lists(&mut l1, &mut l2);

    l1.sort();
    l2.sort();

    for i in 0..l1.len() {
        let v1 = l1[i];
        let v2 = l2[i];

        if v1 >= v2 {
            let diff = v1 - v2;
            sum += diff;
        } else {
            let diff = v2 - v1;
            sum += diff;
        }
    }

    println!("Part 1: The total sum is: {sum}");
}

pub fn dowork2() {
    let mut l1: Vec<i32> = Vec::new();
    let mut l2: Vec<i32> = Vec::new();

    populate_lists(&mut l1, &mut l2);

    let mut map: HashMap<i32, i32> = HashMap::new();

    for val in l2 {
        if map.contains_key(&val) {
            if let Some(x) = map.get_mut(&val){
                *x+= 1;
            }
        }
        else{
            map.insert(val, 1);
        }
    }

    let mut sum = 0;
    for val in l1{
        if map.contains_key(&val){
            sum+= val * map[&val];
        }
    }

    println!("Part 2: Sum is {sum}");
}

pub fn populate_lists(l1: &mut Vec<i32>, l2: &mut Vec<i32>) {
    let f = File::open("C://workspaces//aoc2024//src//input1_1.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);

    for line in reader.lines() {
        let prs = line.unwrap();
        let vals: Vec<&str> = prs.split_whitespace().collect();

        let v1 = vals[0].parse::<i32>().unwrap();
        let v2 = vals[1].parse::<i32>().unwrap();

        l1.push(v1);
        l2.push(v2);
    }
}
