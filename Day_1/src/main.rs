use std::fs::read_to_string;

fn main() {
part_one();
part_two();
}


fn part_one () {
    let mut first_number: Vec<i32> = Vec::new();
    let mut second_number: Vec<i32> = Vec::new();

    let input: String = read_to_string("src/input.txt")
    .expect("Can't find file");

    for line in input.lines() {
        let numbers: Vec<i32> = line
        .split_whitespace()
        .filter_map(|n| n.parse::<i32>().ok())
        .collect();

        if numbers.len() == 2 {

        first_number.push(numbers[0]);
        second_number.push(numbers[1]);


        } else {
        eprintln!("Invalid line format: {}", line);
        }

    }
    first_number.sort();
    second_number.sort();

    let mut answer = 0;

    for i in 0..first_number.len() {
        answer += (first_number[i] - second_number[i]).abs();
        // println!("{} - {}", first_number[i], second_number[i]);
    }


    println!("{}", answer);
}

fn part_two() {
    let mut locations: (Vec<u32>, Vec<u32>) = (vec![], vec![]);

    let mut answer = 0;
    let input: String = read_to_string("src/input.txt")
    .expect("Can't find file");

    for line in input.lines() {
        let numbers: Vec<u32> = line
        .split_whitespace()
        .filter_map(|n| n.parse::<u32>().ok())
        .collect();

        if numbers.len() == 2 {

        locations.0.push(numbers[0]);
        locations.1.push(numbers[1]);


        } else {
        eprintln!("Invalid line format: {}", line);
        }

    }

    for i in 0..locations.0.len() {
        let mut multiplier = 0;
        for j in 0..locations.1.len() {
            
            if locations.0[i] == locations.1[j] {
                multiplier += 1 
            }
        }
        answer += locations.0[i] * multiplier;
    }

    println!("{}", answer);
}