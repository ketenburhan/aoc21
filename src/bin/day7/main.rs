use std::{collections::HashMap, str::FromStr};

const DATA: &str = include_str!("data.txt");

struct Crabs(HashMap<u16, usize>);
impl FromStr for Crabs {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut out = HashMap::new();
        s.trim_end()
            .split(',')
            .map(|num_str| num_str.parse::<u16>().unwrap())
            .for_each(|pos| {
                let count = out.entry(pos).or_insert(0);
                *count += 1;
            });
        Ok(Self(out))
    }
}

fn main() {
    let crabs = Crabs::from_str(DATA).unwrap();

    println!("{:#?}", crabs.0);

    println!("part1: {}", answer(1, &crabs));
    println!("part2: {}", answer(2, &crabs));
}

fn answer(part: u8, crabs: &Crabs) -> usize {
    (0..=2000)
        .into_iter()
        .map(|pos| {
            crabs
                .0
                .iter()
                .map(|(&old_pos, &count)| {
                    let diff = if old_pos > pos {
                        old_pos - pos
                    } else {
                        pos - old_pos
                    };
                    match part {
                        1 => diff as usize * count,
                        2 => match diff {
                            n if n > 1 => (1..=n as usize).sum::<usize>() * count,
                            1 => count,
                            _ => 0,
                        },
                        _ => unreachable!(),
                    }
                })
                .sum()
        })
        .min()
        .unwrap()
}
