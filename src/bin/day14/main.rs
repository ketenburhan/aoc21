use std::collections::HashMap;

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part1: {}", part1(DATA));
    println!("part2: {}", part2(DATA));
}

fn part1(data: &str) -> usize {
    let steps = 10;
    let mut lines = data.lines();
    let mut poly: Vec<char> = lines.next().unwrap().chars().collect();

    lines.next().unwrap();

    let mut formulas: HashMap<(char, char), char> = HashMap::new();
    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        formulas.insert((chars[0], chars[1]), chars[6]);
    }

    for _ in 0..steps {
        for i in (1..poly.len()).rev() {
            if let Some(x) = formulas.get(&(poly[i - 1], poly[i])) {
                poly.insert(i, *x);
            }
        }
    }
    let mut counter: HashMap<char, usize> = HashMap::new();
    for &c in poly.iter() {
        let count = counter.entry(c).or_insert(0);
        *count += 1;
    }

    let values = counter.values();
    println!("{:#?}", counter);
    values.clone().max().unwrap() - values.min().unwrap()
}

fn part2(_data: &str) -> usize {
    todo!("gived up for now")
}
