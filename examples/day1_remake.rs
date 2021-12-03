use std::fs;
use std::str::Lines;

fn main() {
    let contents =
        fs::read_to_string("./inputs/day1.txt").expect("Something went wrong reading the file");

    let lines = contents.lines();
    println!("{}", part1(lines.clone()));
    println!("{}", part2_new(lines));
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

fn part2_new(lines: Lines) -> i32 {
    let numbers = lines
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut count = 0;

    for index in 3..numbers.len() {
        if numbers[index] > numbers[index - 3] {
            count += 1;
        }
    }

    count
}
