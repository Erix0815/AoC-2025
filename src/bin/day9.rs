use std::{fmt::Debug, fs, num::ParseIntError, str::FromStr, usize};

#[derive(PartialEq, Eq, Hash)]
struct Point { x: i64, y: i64 }
enum PointParseErr {
    IllegalDimensionCount(usize),
    IllegalDimension(ParseIntError)
}
impl From<ParseIntError> for PointParseErr {
    fn from(value: ParseIntError) -> Self {Self::IllegalDimension(value)}
}

impl FromStr for Point {
    type Err = PointParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords = s.split(',').map(|xy| xy.trim().parse::<i64>()).collect::<Result<Vec<_>,_>>()?;
        if coords.len() != 2 {Err(PointParseErr::IllegalDimensionCount(coords.len()))}
        else {Ok(Point{x: coords[0], y: coords[1] })}
    }
}

impl Debug for PointParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           PointParseErr::IllegalDimensionCount(num) => write!(f, "Wrong number of dimensions for Point, expected 2, found {num}"),
           PointParseErr::IllegalDimension(inner) => write!(f, "Error parsing dimension in Point: {inner}")
       }
    }
}

fn area_in_rect(frist: &Point, recond: &Point) -> i64 {
    ((frist.x - recond.x).abs() + 1)*((frist.y - recond.y).abs() + 1)
}

fn pairs<T>(vec: &Vec<T>) -> impl Iterator<Item = (&T, &T)> {
    (0..vec.len()).flat_map(move |i| 
        (i+1..vec.len()).map(move |j|
            (&vec[i], &vec[j])
        )
    )
}

fn main() {
    const FILE: &str = "./input/day9.txt";
    let content = fs::read_to_string(FILE)
        .expect(format!("failed to read file: '{FILE}'").as_str());

    let points = content.lines().map(|line|
        line.parse::<Point>()
        .expect(format!("invalid point '{line}'").as_str())
    ).collect::<Vec<_>>();
    
    let possible_rects = pairs(&points);
    let largest = possible_rects.max_by_key(|&(p1, p2)|
            area_in_rect(p1, p2))
        .expect("not a single possible rect");
    let area = area_in_rect(largest.0, largest.1);
    
    println!("the area of the largest react is {area}");
}
