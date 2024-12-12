use std::{
    char,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn dowork() {
    let pzl = get_data();
    let mut count = 0;

    // cartesian plane
    for y in 0..pzl.len() {
        for x in 0..pzl[y].len() {
            if pzl[y][x] == 'X' {
                // start search
                count += search(y, x, next_char('X'), &pzl);
            }
        }
    }
    println!("count is {count}")
}

fn search(y: usize, x: usize, c: char, pzl: &Vec<Vec<char>>) -> i32 {
    let mut found = 0;

    found += up(y, x, c, &pzl);
    found += down(y, x, c, &pzl);
    found += right(y, x, c, &pzl);
    found += left(y, x, c, &pzl);
    found += down_l(y, x, c, &pzl);
    found += down_r(y, x, c, &pzl);
    found += up_l(y, x, c, &pzl);
    found += up_r(y, x, c, &pzl);

    return found;
}

fn search_cardinal(y: usize, x: usize, c: char, cardinality: &str, pzl: &Vec<Vec<char>>){
    let y_add = get_y_add_for_cardinality(cardinality);
    let x_add = get_x_add_for_cardinality(cardinality);

    // if y <= 0 {
    //     return 0;
    // }

    // if char_at_equals(y - 1, x, c, pzl) {
    //     if c == 'S' {
    //         return 1;
    //     } else {
    //         return up(y - 1, x, next_char(c), pzl);
    //     }
    // } else {
    //     return 0;
    // }
}

fn get_y_add_for_cardinality(cardinality: &str) -> i32{
    match cardinality{
        "u" | "u_l" | "u_r"=> return -1,
        "d" | "d_l" | "d_r" => return 1,
        "l"| "r" => return 0,
        _ => return i32::MIN
    }
}

fn get_x_add_for_cardinality(cardinality: &str) -> i32{
    match cardinality{
        "l" | "u_l" | "d_l" => return -1,
        "r" | "u_r" | "d_r" => return 1,
        "u"| "d" => return 0,
        _ => return i32::MIN
    }
}

fn up(y: usize, x: usize, c: char, pzl: &Vec<Vec<char>>) -> i32 {
    if y <= 0 {
        return 0;
    }

    if char_at_equals(y - 1, x, c, pzl) {
        if c == 'S' {
            return 1;
        } else {
            return up(y - 1, x, next_char(c), pzl);
        }
    } else {
        return 0;
    }
}
fn down(y: usize, x: usize, c: char, pzl: &Vec<Vec<char>>) -> i32 {
    if y <= pzl.len() {
        return 0;
    }

    if char_at_equals(y + 1, x, c, pzl) {
        if c == 'S' {
            return 1;
        } else {
            return down(y + 1, x, next_char(c), pzl);
        }
    } else {
        return 0;
    }
}
fn left(y: usize, x: usize, c: char, pzl: &Vec<Vec<char>>) -> i32 {
    if x <= 0 {
        return 0;
    }

    if char_at_equals(y, x - 1, c, pzl) {
        if c == 'S' {
            return 1;
        } else {
            return left(y, x - 1, next_char(c), pzl);
        }
    } else {
        return 0;
    }
}
fn right(y: usize, x: usize, c: char, pzl: &Vec<Vec<char>>) -> i32 {
    if x >= pzl[y].len() - 1 {
        return 0;
    }

    if char_at_equals(y, x + 1, c, pzl) {
        if c == 'S' {
            return 1;
        } else {
            return right(y, x + 1, next_char(c), pzl);
        }
    } else {
        return 0;
    }
}
fn up_l(y: usize, x: usize, c: char, pzl: &Vec<Vec<char>>) -> i32 {
    if y <= 0 || x <= 0 {
        return 0;
    }

    if char_at_equals(y - 1, x - 1, c, pzl) {
        if c == 'S' {
            return 1;
        } else {
            return up_l(y - 1, x - 1, next_char(c), pzl);
        }
    } else {
        return 0;
    }
}
fn up_r(y: usize, x: usize, c: char, pzl: &Vec<Vec<char>>) -> i32 {
    if y <= 0 || x > pzl[y].len() - 1 {
        return 0;
    }

    if char_at_equals(y - 1, x + 1, c, pzl) {
        if c == 'S' {
            return 1;
        } else {
            return up_r(y - 1, x + 1, next_char(c), pzl);
        }
    } else {
        return 0;
    }
}
fn down_l(y: usize, x: usize, c: char, pzl: &Vec<Vec<char>>) -> i32 {
    if y > pzl.len() - 1 || x <= 0 {
        return 0;
    }

    if char_at_equals(y + 1, x - 1, c, pzl) {
        if c == 'S' {
            return 1;
        } else {
            return up(y + 1, x - 1, next_char(c), pzl);
        }
    } else {
        return 0;
    }
}
fn down_r(y: usize, x: usize, c: char, pzl: &Vec<Vec<char>>) -> i32 {
    if y > pzl.len() - 1 ||  x > pzl[y].len() {
        return 0;
    }

    if char_at_equals(y + 1, x+1, c, pzl) {
        if c == 'S' {
            return 1;
        } else {
            return up(y + 1, x+1, next_char(c), pzl);
        }
    } else {
        return 0;
    }
}

fn char_at_equals(i: usize, j: usize, c: char, pzl: &Vec<Vec<char>>) -> bool {
    return pzl[i][j] == c;
}

fn next_char(c: char) -> (char) {
    if c == 'X' {
        return 'M';
    } else if c == 'M' {
        return 'A';
    } else if c == 'A' {
        return 'S';
    } else if c == 'S' {
        return 'd';
    } else {
        return 'd';
    }
}

pub fn dowork2() {}

fn get_data() -> Vec<Vec<char>> {
    let f = File::open("C:/workspaces/aoc2024/src/day_4/src/input.txt").unwrap();
    let reader = BufReader::new(f);

    let mut pzl: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        pzl.push(match line {
            Ok(s) => s.chars().collect(),
            Err(_s) => continue,
        });
    }
    return pzl;
}
