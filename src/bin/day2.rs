use std::fs;

fn is_palindrome(n: &i128) -> bool{
    let repr = format!("{n}");
    let end = repr.len();
    if end % 2 != 0 {return false}
    let half = end/2;
    repr[..half] == repr[half..]
}

fn main() {
    const FILE: &str = "./input/day2.txt";
    let mut sum = 0;
    let content = fs::read_to_string(FILE)
        .expect(format!("failed to read file: '{FILE}'").as_str());
    for pair  in content.split(','){
        let (from_str, to_str) = pair.split_once('-')
            .expect(format!("encountered illegal pair: '{pair}'").as_str());
        let from = from_str.trim().parse::<i128>()
            .expect(format!("could not parse start-id: '{from_str}'").as_str());
        let to = to_str.trim().parse::<i128>()
            .expect(format!("could not parse end-id: '{to_str}'").as_str());
        let ids = from..=to;
        let invalid_ids = ids.filter(is_palindrome);
        sum += invalid_ids.sum::<i128>();
    }

    println!("the sum of invalid ids is '{sum}'");
}
