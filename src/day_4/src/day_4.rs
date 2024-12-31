use std::{
    default,
    fs::File,
    io::{BufRead, BufReader},
};
extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

extern crate helpers;
use helpers::helpers::{check_bounds, get_x_add_for_cardinality, get_y_add_for_cardinality};

pub fn dowork() {
    let pzl = get_data();
    let mut count = 0;

    // cartesian plane
    for y in 0..pzl.len() {
        for x in 0..pzl[y].len() {
            if pzl[y][x] == "X" {
                // start search
                count += search(y, x, next_char("X"), &pzl);
            }
        }
    }
    println!("count is {count}")
}

pub fn dowork2() {
    let pzl = get_data();
    let mut count = 0;

    // cartesian plane
    for y in 0..pzl.len() {
        for x in 0..pzl[y].len() {
            if pzl[y][x] == "A" {
                // start search
                count += search_x(y, x, &pzl);
            }
        }
    }
    println!("count is {count}")
}

fn search(y: usize, x: usize, c: &str, pzl: &Vec<Vec<String>>) -> i32 {
    let mut found = 0;

    found += search_cardinal(y, x, c, "u", &pzl);
    found += search_cardinal(y, x, c, "d", &pzl);
    found += search_cardinal(y, x, c, "r", &pzl);
    found += search_cardinal(y, x, c, "l", &pzl);
    found += search_cardinal(y, x, c, "d_l", &pzl);
    found += search_cardinal(y, x, c, "d_r", &pzl);
    found += search_cardinal(y, x, c, "u_l", &pzl);
    found += search_cardinal(y, x, c, "u_r", &pzl);

    return found;
}

fn search_x(y: usize, x: usize, pzl: &Vec<Vec<String>>) -> i32 {
    let mut found = 0;
    //let mut found_chars = ["a"; 4];
    let mut used: &str = "";

    found += search_cardinal_x(y, x, &used, "d_l", &pzl);
    found += search_cardinal_x(y, x, &used, "u_r", &pzl);

    used = "";

    found += search_cardinal_x(y, x, &used, "d_r", &pzl);
    found += search_cardinal_x(y, x, &used, "u_l", &pzl);

    return if found == 4 { 1 } else { 0 };
}

fn search_cardinal(y: usize, x: usize, c: &str, cardinality: &str, pzl: &Vec<Vec<String>>) -> i32 {
    if !check_bounds(y, x, cardinality, pzl) {
        return 0;
    }

    let y_add = get_y_add_for_cardinality(cardinality);
    let x_add = get_x_add_for_cardinality(cardinality);

    if char_at_equals(
        (y as i32 + y_add) as usize,
        (x as i32 + x_add) as usize,
        c,
        pzl,
    ) {
        if c == "S" {
            return 1;
        } else {
            return search_cardinal(
                (y as i32 + y_add) as usize,
                (x as i32 + x_add) as usize,
                next_char(c),
                cardinality,
                pzl,
            );
        }
    } else {
        return 0;
    }
}

fn search_cardinal_x(
    y: usize,
    x: usize,
    mut used: &str,
    cardinality: &str,
    pzl: &Vec<Vec<String>>,
) -> i32 {
    if !check_bounds(y, x, cardinality, pzl) {
        return 0;
    }

    let y_new = (y as i32 + get_y_add_for_cardinality(cardinality)) as usize;
    let x_new = (x as i32 + get_x_add_for_cardinality(cardinality)) as usize;

    let next_char = next_char_x(used);
    if next_char.len() == 2 {
        let mut char = next_char[0];
        if char_at_equals(y_new, x_new, char, pzl) {
            used = char;
            return 1;
        }

        char = next_char[1];
        if char_at_equals(y_new, x_new, char, pzl) {
            used = char;
            return 1;
        }
        // didn't find
        else {
            return 0;
        }
    } else {
        return if char_at_equals(y_new, x_new, next_char[0], pzl) {
            1
        } else {
            0
        };
    }
}

fn char_at_equals(y: usize, x: usize, c: &str, pzl: &Vec<Vec<String>>) -> bool {
    return pzl[y][x] == c;
}

fn next_char(c: &str) -> &str {
    if c == "X" {
        return "M";
    } else if c == "M" {
        return "A";
    } else if c == "A" {
        return "S";
    } else if c == "S" {
        return "d";
    } else {
        return "d";
    }
}

fn next_char_x(c: &str) -> Vec<&str> {
    let mut vals: Vec<&str> = Vec::new();
    if c == "" {
        vals.push("M");
        vals.push("S");
    } else if c == "M" {
        vals.push("S");
    } else if c == "S" {
        vals.push("M");
    }
    return vals;
}

fn get_data() -> Vec<Vec<String>> {
    let f = File::open("C:/workspaces/aoc2024/src/day_4/src/input.txt").unwrap();
    let reader = BufReader::new(f);

    let mut pzl: Vec<Vec<String>> = Vec::new();
    for line in reader.lines() {
        pzl.push(match line {
            Ok(s) => s.graphemes(true).map(|s| s.to_string()).collect(),
            Err(_s) => continue,
        });
    }
    return pzl;
}
