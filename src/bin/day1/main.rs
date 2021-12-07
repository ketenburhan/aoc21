use std::str::Lines;

const DATA: &str = include_str!("data.txt");

fn main() {
    let lines = DATA.lines();
    println!("part1: {}", part1(lines.clone()));
    println!("part2: {}", part2_new(lines));
}

fn part1(mut lines: Lines) -> i32 {
    let mut prev: i32 = lines.next().unwrap().parse().unwrap();
    let mut count = 0;
    for line in lines {
        let num: i32 = line.parse().unwrap();
        if prev < num {
            count += 1;
        }
        prev = num;
    }
    count
}

#[allow(dead_code)]
fn part2(lines: Lines) -> i32 {
    let numbers: Vec<i32> = lines.map(|x| x.parse::<i32>().unwrap()).collect();
    let mut prev = 0;
    let mut count = -1;
    for i in 0..numbers.len() {
        let (n0, n1, n2) = (numbers.get(i), numbers.get(i + 1), numbers.get(i + 2));
        if n2.is_none() {
            break;
        }
        let sum = n0.unwrap() + n1.unwrap() + n2.unwrap();
        if sum > prev {
            count += 1;
        }
        prev = sum;
    }
    count
}

fn part2_new(lines: Lines) -> i32 {
    let mut numbers0 = lines.map(|x| x.parse::<i32>().unwrap());
    let mut numbers1 = numbers0.clone();
    let numbers2 = numbers0.clone();

    numbers0.next();
    numbers0.next();
    numbers1.next();
    let mut prev = 0;

    let mut count = -1;

    for (n0, (n1, n2)) in numbers2.zip(numbers1.zip(numbers0)) {
        let sum = n0 + n1 + n2;
        if sum > prev {
            count += 1;
        }
        prev = sum;
    }
    count
}
