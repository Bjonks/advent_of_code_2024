use std::fs::read_to_string;

fn main() {

    let mut answer_part_one: u32 = 0;
    let mut answer_part_two: u32 = 0;
    let input = read_to_string("./src/input.txt")
    .expect("Can't find file");

    let values = prepare_input(input);
    
    for i in 0..values.len() {

        if check_lines_part_one(&values[i]) == true {
            answer_part_one += 1
        } 
        if check_lines_part_two(values[i].clone()) == true {
            answer_part_two += 1
        }
    }

    println!("Answer for part one is: {}", answer_part_one);
    println!("Answer for part two is: {}", answer_part_two);
}

fn check_lines_part_one (line: &Vec<i32>) -> bool {

    let direction = line[1] - line[0];
    if direction == 0 || direction.abs() > 3 {
        return false
    }

    let is_increasing = direction > 0;


    for i in 1..line.len() {
        let diff = line[i] - line[i - 1];

        if diff.abs() < 1 || diff.abs() > 3 || (is_increasing && diff < 0) || (!is_increasing && diff > 0) {
            return false
        }
    }

    true
}

fn check_lines_part_two (mut line: Vec<i32>) -> bool {

    let mut fail_dampener_used:bool = false;
    let direction = line[1] - line[0];
    if direction == 0 || direction.abs() > 3 {
        fail_dampener_used = true;
    }

    let is_increasing = direction > 0;


    for i in 1..line.len() {

        let diff = line[i] - line[i - 1];

        if diff.abs() < 1 || diff.abs() > 3 || (is_increasing && diff < 0) || (!is_increasing && diff > 0) {
            if fail_dampener_used == true {
                return false
            }
            fail_dampener_used = true;
            line.remove(i);
            
        }
    }

    true
}

fn prepare_input(input: String) -> Vec<Vec<i32>> {


    let lines: Vec<Vec<i32>> = input
    .lines()
    .map(|line| {
        line.split_whitespace()
        .filter_map(|num|num.parse::<i32>().ok())
        .collect()
    })
    .collect();

    return lines;
}
