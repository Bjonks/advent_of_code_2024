use std::fs::read_to_string;

fn main() {

    let input = read_to_string("./src/input.txt")
    .expect("Can't find file");

    let values = prepare_input(input);
    
    for i in 0..values.len() {
        
    }
}

fn prepare_input(input: String) -> Vec<Vec<u32>> {


    let lines: Vec<Vec<u32>> = input
    .lines()
    .map(|line| {
        line.split_whitespace()
        .filter_map(|num|num.parse::<u32>().ok())
        .collect()
    })
    .collect();

    return lines;
}
