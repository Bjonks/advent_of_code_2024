use std::fs::read_to_string;

fn main() {
    let mut answer_part_one: u32 = 0;
    let mut answer_part_two: u32 = 0;
    let input = read_to_string("./src/input.txt").expect("Can't find file");

    let values = prepare_input(input);

    for i in 0..values.len() {
        if check_lines(&values[i]) == true {
            answer_part_one += 1
        } else if fail_dampener(&values[i]) == true {
            answer_part_two += 1
        }
    }

    answer_part_two = answer_part_one + answer_part_two;

    println!("Answer for part one is: {}", answer_part_one);
    println!("Answer for part two is: {}", answer_part_two);
}

fn check_lines(line: &Vec<i32>) -> bool {
    let direction = line[1] - line[0];
    if direction == 0 || direction.abs() > 3 {
        return false;
    }

    let is_increasing = direction > 0;

    for i in 1..line.len() {
        let diff = line[i] - line[i - 1];

        if diff.abs() < 1
            || diff.abs() > 3
            || (is_increasing && diff < 0)
            || (!is_increasing && diff > 0)
        {
            return false;
        }
    }

    true
}

fn fail_dampener(line: &Vec<i32>) -> bool {
    for i in 0..line.len() {
        let mut modified_line = line.clone();
        modified_line.remove(i);

        if check_lines(&modified_line) == true {
            return true;
        }
    }
    return false;
}

fn prepare_input(input: String) -> Vec<Vec<i32>> {
    let lines: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect()
        })
        .collect();

    return lines;
}
