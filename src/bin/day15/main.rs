use std::collections::HashMap;
const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part1: {}", part1(DATA));
    println!("part2: {}", part2(DATA));
}

fn part1(data: &str) -> usize {
    let map: Vec<Vec<usize>> = data
        .lines()
        .map(|line| line.bytes().map(|n| n as usize - 48).collect())
        .collect();
    let mut memory: HashMap<(usize, usize), usize> = HashMap::new();
    part1_solve(&map, &mut memory, 0, 0) - map[0][0]
}
fn part1_solve(
    map: &[Vec<usize>],
    memory: &mut HashMap<(usize, usize), usize>,
    x: usize,
    y: usize,
) -> usize {
    if let Some(&x) = memory.get(&(x, y)) {
        return x;
    }
    if y >= map.len() || x >= map.len() {
        return usize::MAX;
    }
    if x == map.len() - 1 && y == map.len() - 1 {
        return map[y][x];
    }
    let out = map[y][x]
        + [
            part1_solve(map, memory, x, y + 1),
            part1_solve(map, memory, x + 1, y),
        ]
        .iter()
        .min()
        .unwrap();
    memory.insert((x, y), out);
    out
}

fn part2(_data: &str) -> usize {
    0
}
