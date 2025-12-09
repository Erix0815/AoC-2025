use std::fs;

fn biggest_digit_and_index(bank: &str) -> (i32, usize) {
    bank.char_indices()
    .map(|(index, char)|
        (char.to_string().parse().expect("unecpected non-digit"), index)
    )
    .max_by(|first, second|
        if first.0 < second.0 {return std::cmp::Ordering::Less;}
        else if first.0 > second.0 {return std::cmp::Ordering::Greater;}
        else if first.1 <= second.1 {return std::cmp::Ordering::Greater;}
        else {return std::cmp::Ordering::Less;}
    ).expect(format!("invalid, empty bank-slice: '{bank}'").as_str())
}

fn max_joltage(bank: &str) -> i32 {
    let (tens, cutoff) = biggest_digit_and_index(&bank[..&bank.len()-1]);
    let (ones, _) = biggest_digit_and_index(&bank[cutoff+1..]);
    10*tens + ones
}

fn main() {
    const FILE: &str = "./input/day3.txt";
    let content = fs::read_to_string(FILE)
        .expect(format!("failed to read file: '{FILE}'").as_str());
    let result: i32 = content.lines().map(max_joltage).sum();
    println!("the total joltage is {result}")
}
