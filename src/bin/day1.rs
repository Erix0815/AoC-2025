use std::fs;

fn rotation_from_line(line: &str) -> i32 {
    let dir_char = line.chars().nth(0).expect("invalid empty line");
    let dir = match dir_char {
        'L' => -1,
        'R' => 1,
        _ => panic!("invalid direction '{dir_char}' in line '{line}'")
    };
    let steps = line[1..].parse::<i32>()
        .expect(format!("could not parse steps from rotation: '{line}'").as_str());
    dir * steps
}

fn main() {
    const FILE: &str = "./input/day1.txt";
    let mut pw = 0;
    let mut pos = 50;
    const POSITIONS: i32 = 100;
    let content = fs::read_to_string(FILE)
        .expect(format!("failed to read file: '{FILE}'").as_str());
    let rotations = content.lines().map(rotation_from_line);

    for rotation in rotations{
        pos = (pos + rotation) % POSITIONS;
        if pos == 0 {pw += 1;}
    }

    println!("password is {pw}")
}
