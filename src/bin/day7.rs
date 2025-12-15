use std::{collections::HashSet, fs};

fn main() {
    const FILE: &str = "./input/day7.txt";
    let mut splits = 0;
    let content = fs::read_to_string(FILE)
        .expect(format!("failed to read file: '{FILE}'").as_str());
    let mut lines = content.lines();
    let initial = lines.next().expect("input did not contain a single line");
    let mut tachyons = initial.char_indices()
        .filter_map(|(index, char)|
            if char == 'S' {Some(index)}
            else {None}
        ).collect::<HashSet<_>>();
    for line in lines {
        let intersections = line.char_indices()
            .filter_map(|(index, char)|
                if tachyons.contains(&index) && char == '^' {Some(index)}
                else {None}
            ).collect::<Vec<_>>();
        splits += intersections.len();
        for intersection in intersections {
            tachyons.remove(&intersection);
            tachyons.insert(intersection+1);
            tachyons.insert(intersection-1);
        }
    }
    println!("the tachyon-beam was split {splits} times");
}
