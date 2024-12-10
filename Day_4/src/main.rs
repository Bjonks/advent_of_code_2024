use std::{arch::x86_64::_MM_ROUND_DOWN, fs::read_to_string};

fn main() {

    let input: String = read_to_string("./src/input.txt").unwrap();
    let mut answer_part_one = 0;
    let grid: Vec<Vec<char>> = input.lines()
    .map(|line| line.chars().collect())
    .collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let word = "XMAS";
    let word_rev = word.chars().rev().collect::<String>();

for row in grid.iter() {
    for window in row.windows(4) {
        let check = window.iter().collect::<String>();
        if check == word || check == word_rev {
            answer_part_one += 1;
        }
    }
}

for col in 0..cols {
    for row in 0..rows - 3 {
        let check = (0..=3).map(|i| grid[row + i][col]).collect::<String>();
        if check == word || check == word_rev {
            answer_part_one += 1;
        }
    }
}
println!("{}", answer_part_one);
}