use std::fs;

fn is_roll(rolls: &Vec<Vec<bool>>, row: i32, col: i32) -> bool {
    match row {
        r if r < 0 => false,
        r if r >= rolls.len() as i32 => false,
        r => match col {
            c if c < 0 => false,
            c if c >= rolls[r as usize].len() as i32 => false,
            c => rolls[r as usize][c as usize],
        },
    }
}

fn is_movable(rolls: &Vec<Vec<bool>>, row: i32, col: i32) -> bool {
    let mut neighbor_rolls = 0;
    if is_roll(&rolls, row - 1, col - 1) {neighbor_rolls += 1}
    if is_roll(&rolls, row - 1, col + 0) {neighbor_rolls += 1}
    if is_roll(&rolls, row - 1, col + 1) {neighbor_rolls += 1}
    if is_roll(&rolls, row + 0, col - 1) {neighbor_rolls += 1}
    if is_roll(&rolls, row + 0, col + 1) {neighbor_rolls += 1}
    if is_roll(&rolls, row + 1, col - 1) {neighbor_rolls += 1}
    if is_roll(&rolls, row + 1, col + 0) {neighbor_rolls += 1}
    if is_roll(&rolls, row + 1, col + 1) {neighbor_rolls += 1}
    neighbor_rolls < 4
}

fn main() {
    const FILE: &str = "./input/day4.txt";
    let mut movable_rolls = 0;
    let content =
        fs::read_to_string(FILE).expect(format!("failed to read file: '{FILE}'").as_str());

    let rolls = content.lines().map(|line| {
            line.chars().map(|char| match char {
                    '.' => false,
                    '@' => true,
                     _  => panic!("illegal character in input: '{char}'"),
                }).collect::<Vec<_>>()
        }).collect::<Vec<_>>();

    for (row, row_index) in rolls.iter().zip(0..) {
        for (roll, col_index) in row.iter().zip(0..) {
            if *roll && is_movable(&rolls, row_index, col_index) {movable_rolls += 1}
        }
    }

    println!("there are {movable_rolls} movable rolls");
}
