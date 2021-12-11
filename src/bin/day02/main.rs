use std::str::Lines;

const DATA: &str = include_str!("data.txt");

fn main() {
    let lines = DATA.lines();
    println!("part1: {}", part1(lines.clone()));
    println!("part2: {}", part2(lines));
}

fn part1(lines: Lines) -> i32 {
    let mut x = 0;
    let mut y = 0;

    for line in lines {
        let mut splitted = line.split(' ');
        let cmd = splitted.next().unwrap();
        let num: i32 = splitted.next().unwrap().parse().unwrap();

        match cmd {
            "forward" => x += num,
            "down" => y += num,
            "up" => y -= num,
            _ => unreachable!(),
        }
    }
    x * y
}

fn part2(lines: Lines) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    for line in lines {
        let mut splitted = line.split(' ');
        let cmd = splitted.next().unwrap();
        let num: i32 = splitted.next().unwrap().parse().unwrap();

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
    x * y
}
