use std::fs::read_to_string;

use regex::Regex;

fn main() {
    
    let mut answer_part_one: u32 = 0;

    let input = read_to_string("./src/input.txt").unwrap();
    let re_part_one = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let re_part_two = Regex::new(r"(do\(\)|don't\(\)|mul\((\d+),(\d+)\))").unwrap();

    for cap in re_part_one.captures_iter(&input) {

        let num1: u32 = cap[1].parse().unwrap();
        let num2: u32 = cap[2].parse().unwrap();

        answer_part_one += num1 * num2;
    }



    println!("{}", answer_part_one);
}