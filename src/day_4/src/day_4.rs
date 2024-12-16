use std::{
    fs::File,
    io::{BufRead, BufReader},
};
extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

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

fn search_cardinal(y: usize, x: usize, c: &str, cardinality: &str, pzl: &Vec<Vec<String>>) -> i32 {
    let y_add = get_y_add_for_cardinality(cardinality);
    let x_add = get_x_add_for_cardinality(cardinality);

    if y >= pzl.len() || x >= pzl[y].len() {
        return 0;
    }

    if char_at_equals(y, x, c, pzl) {
        if c == "S" {
            return 1;
        } else {
            // don't go out of bounds
            if !check_bounds(y, x, cardinality, pzl) {
                return 0;
            }
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

fn check_bounds(y: usize, x: usize, cardinality: &str, pzl: &Vec<Vec<String>>) -> bool {
    if y == 0 && get_y_add_for_cardinality(cardinality) == -1 {
        return false;
    } else if y == pzl.len() - 1 && get_y_add_for_cardinality(cardinality) == 1 {
        return false;
    } else if x == 0 && get_x_add_for_cardinality(cardinality) == -1 {
        return false;
    } else if x == pzl[y].len() - 1 && get_x_add_for_cardinality(cardinality) == 1 {
        return false;
    }
    return true;
}

fn get_y_add_for_cardinality(cardinality: &str) -> i32 {
    match cardinality {
        "u" | "u_l" | "u_r" => return -1,
        "d" | "d_l" | "d_r" => return 1,
        "l" | "r" => return 0,
        _ => return i32::MIN,
    }
}

fn get_x_add_for_cardinality(cardinality: &str) -> i32 {
    match cardinality {
        "l" | "u_l" | "d_l" => return -1,
        "r" | "u_r" | "d_r" => return 1,
        "u" | "d" => return 0,
        _ => return i32::MIN,
    }
}

fn char_at_equals(y: usize, x: usize, c: &str, pzl: &Vec<Vec<String>>) -> bool {
    if y >= pzl.len() || x >= pzl[y].len() {
        println!("uh oh");
    }
    return pzl[y][x] == c;
}

fn next_char(c: &str) -> (&str) {
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

pub fn dowork2() {}

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

fn up(y: usize, x: usize, c: &str, pzl: &Vec<Vec<String>>) -> i32 {
    if y <= 0 {
        return 0;
    }

    if char_at_equals(y, x, c, pzl) {
        if c == "S" {
            return 1;
        } else {
            return up(y - 1, x, next_char(c), pzl);
        }
    } else {
        return 0;
    }
}
fn down(y: usize, x: usize, c: &str, pzl: &Vec<Vec<String>>) -> i32 {
    if y <= pzl.len() {
        return 0;
    }

    if char_at_equals(y, x, c, pzl) {
        if c == "S" {
            return 1;
        } else {
            return down(y + 1, x, next_char(c), pzl);
        }
    } else {
        return 0;
    }
}
fn left(y: usize, x: usize, c: &str, pzl: &Vec<Vec<String>>) -> i32 {
    if x <= 0 {
        return 0;
    }

    if char_at_equals(y, x, c, pzl) {
        if c == "S" {
            return 1;
        } else {
            return left(y, x - 1, next_char(c), pzl);
        }
    } else {
        return 0;
    }
}
fn right(y: usize, x: usize, c: &str, pzl: &Vec<Vec<String>>) -> i32 {
    if x >= pzl[y].len() {
        return 0;
    }

    if char_at_equals(y, x, c, pzl) {
        if c == "S" {
            return 1;
        } else {
            return right(y, x + 1, next_char(c), pzl);
        }
    } else {
        return 0;
    }
}
fn up_l(y: usize, x: usize, c: &str, pzl: &Vec<Vec<String>>) -> i32 {
    if y <= 0 || x <= 0 {
        return 0;
    }

    if char_at_equals(y, x, c, pzl) {
        if c == "S" {
            return 1;
        } else {
            return up_l(y - 1, x - 1, next_char(c), pzl);
        }
    } else {
        return 0;
    }
}
fn up_r(y: usize, x: usize, c: &str, pzl: &Vec<Vec<String>>) -> i32 {
    if y <= 0 || x >= pzl[y].len() {
        return 0;
    }

    if char_at_equals(y, x, c, pzl) {
        if c == "S" {
            return 1;
        } else {
            return up_r(y - 1, x + 1, next_char(c), pzl);
        }
    } else {
        return 0;
    }
}
fn down_l(y: usize, x: usize, c: &str, pzl: &Vec<Vec<String>>) -> i32 {
    if y >= pzl.len() || x <= 0 {
        return 0;
    }

    if char_at_equals(y, x, c, pzl) {
        if c == "S" {
            return 1;
        } else {
            return up(y + 1, x - 1, next_char(c), pzl);
        }
    } else {
        return 0;
    }
}
fn down_r(y: usize, x: usize, c: &str, pzl: &Vec<Vec<String>>) -> i32 {
    if y >= pzl.len() || x >= pzl[y].len() {
        return 0;
    }

    if char_at_equals(y, x, c, pzl) {
        if c == "S" {
            return 1;
        } else {
            return up(y + 1, x + 1, next_char(c), pzl);
        }
    } else {
        return 0;
    }
}

