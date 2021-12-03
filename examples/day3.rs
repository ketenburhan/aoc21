use std::fs;
use std::ops::Not;
use std::str::Lines;

fn main() {
    let contents =
        fs::read_to_string("./inputs/day3.txt").expect("Something went wrong reading the file");

    let lines = contents.lines();
    println!("{}", part1(lines.clone()));
    // println!("{}", part2(lines));
}

fn part1(lines: Lines) -> i32 {
    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();

    let line_len_half = lines.clone().count() / 2;

    for i in 0..12 {
        let one_count = lines
            .clone()
            .filter(|line| line.chars().nth(i) == Some('1'))
            .count();
        if one_count > line_len_half {
            gamma_rate.push('1');
            epsilon_rate.push('0');
        } else {
            gamma_rate.push('0');
            epsilon_rate.push('1');
        }
    }
    let gamma_rate = u16::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_rate = u16::from_str_radix(&epsilon_rate, 2).unwrap();
    gamma_rate as i32 * epsilon_rate as i32
}
