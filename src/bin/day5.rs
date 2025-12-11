use std::fs;

fn parse_range(line: &str) -> (i64, i64) {
    let (from_str, to_str) = line.split_once('-')
        .expect(format!("encountered illegal range: '{line}'").as_str());
    let from = from_str.trim().parse::<i64>()
        .expect(format!("could not parse start-id: '{from_str}'").as_str());
    let to = to_str.trim().parse::<i64>()
        .expect(format!("could not parse end-id: '{to_str}'").as_str());
    (from, to)
}

fn is_fresh(id: &i64, ranges: &Vec<(i64, i64)>) -> bool {
    ranges.iter().any(|range| &range.0<=id && &range.1>=id)
}

fn main() {
    const FILE: &str = "./input/day5.txt";
    let content = fs::read_to_string(FILE)
        .expect(format!("failed to read file: '{FILE}'").as_str());

    let (ranges_db, id_db) = content.split_once("\n\n")
        .expect("could not split db in ranges and ids at empty line");

    let ranges = ranges_db.lines().map(parse_range).collect::<Vec<_>>();
    let ids = id_db.lines().map(|line|
        line.parse::<i64>().expect(format!("invalid id: '{line}'").as_str())
    );

    let fresh_ids = ids.filter(|id |is_fresh(id, &ranges));
    let count = fresh_ids.count();

    println!("there are {count} fresh ingredients")
}
