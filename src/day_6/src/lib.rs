use std::{fs::File, io::{BufRead, BufReader}};

extern crate unicode_segmentation; 
use unicode_segmentation::UnicodeSegmentation;

extern crate helpers;
use helpers::helpers::{check_bounds, get_x_add_for_cardinality, get_y_add_for_cardinality};


pub fn dowork(){

}

pub fn dowork2(){

}

pub fn distinct_coordinates_visited(x_start: usize, y_start: usize, cardinality: &str, pzl: Vec<Vec<String>>)-> (Vec<bool>, i32){
    let length = pzl[0].len();

    //2d array as 1d for performance
    let mut visited :Vec<bool> = vec![false;pzl.len()*length];

    return  (visited, 0);
}


pub fn get_data() -> Vec<Vec<String>>{
    let f = File::open("C:/workspaces/aoc2024/src/day_6/src/input.txt").unwrap();
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




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
