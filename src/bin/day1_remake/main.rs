use std::str::Lines;

const DATA: &str = include_str!("data.txt");

fn main() {
    let lines = DATA.lines();
    println!("part1: {}", part1(lines.clone()));
    println!("part2: {}", part2_new(lines));
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
#[allow(dead_code)]
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
