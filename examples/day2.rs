use std::fs;
use std::str::Lines;

fn main() {
    println!("day1!");

    let contents =
        fs::read_to_string("./inputs/day2.txt").expect("Something went wrong reading the file");

    let lines = contents.lines();
    println!("{}", part1(lines.clone()));
    println!("{}", part2(lines));
}

fn part1(lines: Lines) -> i32 {
    let mut x = 0;
    let mut y = 0;

    for line in lines {
        let splitted: Vec<&str> = line.split(' ').collect();
        if let [cmd, num_str] = *splitted.as_slice() {
            let num: i32 = num_str.parse().unwrap();

            match cmd {
                "forward" => x += num,
                "down" => y += num,
                "up" => y -= num,
                _ => unreachable!(),
            }
        }
    }
    x * y
}

fn part2(lines: Lines) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    for line in lines {
        let splitted: Vec<&str> = line.split(' ').collect();
        if let [cmd, num_str] = *splitted.as_slice() {
            let num: i32 = num_str.parse().unwrap();

            match cmd {
                "forward" => {
                    x += num;
                    y += aim * num;
                }
                "down" => aim += num,
                "up" => aim -= num,
                _ => unreachable!(),
            }
        }
    }
    x * y
}