use std::str::FromStr;

const DATA: &str = include_str!("data.txt");

#[derive(Clone, Debug, Default)]
struct Fishes([usize; 9], usize);

impl Fishes {
    fn pass_day(&mut self) {
        let zero_count = self.0[self.1];
        self.1 = (self.1 + 1) % 9;
        self.0[(self.1 + 6) % 9] += zero_count;
    }
    fn count(&self) -> usize {
        self.0.iter().sum()
    }
}
impl FromStr for Fishes {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut out = [0; 9];
        let v: Vec<usize> = s
            .trim_end()
            .split(',')
            .map(|num_str| num_str.parse().unwrap())
            .collect();
        for i in v {
            out[i] += 1;
        }
        Ok(Fishes(out, 0))
    }
}
fn main() {
    let fishes: Fishes = Fishes::from_str(DATA).unwrap();

    println!("part1: {}", part1(fishes.clone()));
    println!("part2: {}", part2(fishes));
}
fn part1(mut fishes: Fishes) -> usize {
    for _ in 0..80 {
        fishes.pass_day();
    }
    fishes.count()
}
fn part2(mut fishes: Fishes) -> usize {
    for _ in 0..256 {
        fishes.pass_day();
    }
    fishes.count()
}
