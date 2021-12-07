use std::str::Lines;

const DATA: &str = include_str!("data.txt");

fn main() {
    let lines = DATA.lines();
    println!("part1: {}", part1(lines.clone()));
    println!("part2: {}", part2(lines));
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
enum RatingType {
    Oxy,
    CO2,
}

fn part2(lines: Lines) -> i32 {
    let oxy = bit_criteria(lines.clone().collect(), 0, RatingType::Oxy);
    let co2 = bit_criteria(lines.collect(), 0, RatingType::CO2);
    let oxy = u16::from_str_radix(oxy, 2).unwrap();
    let co2 = u16::from_str_radix(co2, 2).unwrap();
    oxy as i32 * co2 as i32
}

fn bit_criteria(lines: Vec<&str>, i: usize, rating_type: RatingType) -> &str {
    let mut zeros = vec![];
    let mut ones = vec![];

    lines.iter().for_each(|line| match line.chars().nth(i) {
        Some('0') => zeros.push(*line),
        Some('1') => ones.push(*line),
        _ => unreachable!(),
    });
    let rating_vec = if zeros.len() <= ones.len() {
        match rating_type {
            RatingType::Oxy => ones,
            RatingType::CO2 => zeros,
        }
    } else {
        match rating_type {
            RatingType::Oxy => zeros,
            RatingType::CO2 => ones,
        }
    };
    let rating = if rating_vec.len() == 1 {
        rating_vec.get(0).unwrap()
    } else {
        bit_criteria(rating_vec, i + 1, rating_type)
    };
    rating
}
