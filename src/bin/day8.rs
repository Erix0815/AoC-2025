use std::{collections::HashSet, fmt::Debug, fs, num::ParseIntError, str::FromStr, usize};

#[derive(PartialEq, Eq, Hash)]
struct Point { x: i64, y: i64, z: i64 }
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
        let coords = s.split(',').map(|xyz| xyz.trim().parse::<i64>()).collect::<Result<Vec<_>,_>>()?;
        if coords.len() != 3 {Err(PointParseErr::IllegalDimensionCount(coords.len()))}
        else {Ok(Point{x: coords[0], y: coords[1], z: coords[2]})}
    }
}

impl Debug for PointParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           PointParseErr::IllegalDimensionCount(num) => write!(f, "Wrong number of dimensions for Point, expected 3, found {num}"),
           PointParseErr::IllegalDimension(inner) => write!(f, "Error parsing Dimension in Point: {inner}")
       }
    }
}

impl Point {
    fn dist_to(&self, other: &Point) -> f32 {
        ((
        ((self.x - other.x)*(self.x - other.x)) +
        ((self.y - other.y)*(self.y - other.y)) +
        ((self.z - other.z)*(self.z - other.z))
        ) as f32).sqrt()
    }
}

fn pairs<T>(vec: &Vec<T>) -> impl Iterator<Item = (&T, &T)> {
    (0..vec.len()).flat_map(move |i| 
        (i+1..vec.len()).map(move |j|
            (&vec[i], &vec[j])
        )
    )
}

fn circuit(circuits: &Vec<HashSet<&Point>>, point: &Point) -> usize{
    for (index, circuit) in circuits.iter().enumerate() {
        if circuit.contains(point) { return index; }
    }
    return usize::MAX;
}

fn main() {
    const FILE: &str = "./input/day8.txt";
    let content = fs::read_to_string(FILE)
        .expect(format!("failed to read file: '{FILE}'").as_str());

    let points = content.lines().map(|line|
        line.parse::<Point>()
        .expect(format!("invalid point '{line}'").as_str())
    ).collect::<Vec<_>>();
    let mut circuits = points.iter().map(|p|HashSet::from([p; 1])).collect::<Vec<_>>();
    
    let mut possible_connections = pairs(&points)
        .collect::<Vec<_>>();
    possible_connections.sort_by(|(p11, p12), (p21, p22)|
        (p21.dist_to(&p22)).total_cmp(&(p11.dist_to(&p12))));

    for _ in 0..1000 {
        let shortest = possible_connections.pop().expect("no valid connections left");

        let circuit1 = circuit(&circuits, &shortest.0);
        let circuit2 = circuit(&circuits, &shortest.1);

        if circuit1 == circuit2 {continue;}
        
        let removed = circuits.swap_remove(circuit1);
        // recalculate index, it may have changed due to previous swap_remove
        let circuit2 = circuit(&circuits, &shortest.1);
        circuits[circuit2].extend(removed);
    }

    circuits.sort_by_key(|c|c.len());

    let res: i64 = circuits.iter().rev().take(3).map(|c| c.len() as i64).product();

    println!("the product of circuit-sizes is {res}");
}
