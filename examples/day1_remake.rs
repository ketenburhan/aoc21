use std::fs;
use std::str::Lines;

fn main() {
    println!("day1!");

    let contents =
        fs::read_to_string("./inputs/day1.txt").expect("Something went wrong reading the file");

    let lines = contents.lines();
    println!("{}", part1(lines.clone()));
    println!("{}", part2(lines));
}

fn part1(lines: Lines) -> i32 {
    lines
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|w| w[1] > w[0])
        .count()
        .try_into()
        .unwrap()
}
fn part2(lines: Lines) -> i32 {
    lines
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .windows(3)
        .map(|v| v.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|w| w[1] > w[0])
        .count()
        .try_into()
        .unwrap()
}
