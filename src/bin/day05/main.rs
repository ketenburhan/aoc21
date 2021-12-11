use regex::Regex;

const DATA: &str = include_str!("data.txt");

type Floor = [[u32; 1000]; 1000];
static DEF_FLOOR: Floor = [[0; 1000]; 1000];

fn main() {
    let mut ocean_floor = DEF_FLOOR;

    let reg = Regex::new(r#"(\d+),(\d+) -> (\d+),(\d+)"#).unwrap();
    let captures = reg.captures_iter(DATA);

    let coords: Vec<_> = captures
        .map(|cap| {
            let mut iter = cap.iter();
            iter.next();
            let coords: Vec<usize> = iter
                .map(|s| s.unwrap().as_str().parse::<usize>().unwrap())
                .collect();
            (coords[0], coords[1], coords[2], coords[3])
        })
        .collect();
    coords.iter().for_each(|&(x1, y1, x2, y2)| {
        if x1 == x2 {
            let (y_min, y_max) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
            #[allow(clippy::needless_range_loop)]
            for i in y_min..=y_max {
                ocean_floor[i][x1] += 1;
            }
        } else if y1 == y2 {
            let (x_min, x_max) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
            for i in x_min..=x_max {
                ocean_floor[y1][i] += 1;
            }
        }
    });
    println!("part1: {}", part1(&ocean_floor));
    coords.iter().for_each(|&(x1, y1, x2, y2)| {
        let (y_min, y_max) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
        let (x_min, x_max) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
        if x_max - x_min == y_max - y_min {
            let x_range: Vec<usize> = if x1 == x_min {
                (x1..=x2).collect()
            } else {
                (x2..=x1).rev().collect()
            };
            let y_range: Vec<usize> = if y1 == y_min {
                (y1..=y2).collect()
            } else {
                (y2..=y1).rev().collect()
            };

            for (&x, &y) in x_range.iter().zip(&y_range) {
                ocean_floor[y][x] += 1;
            }
        }
    });
    println!("part2: {}", part1(&ocean_floor));
}
fn part1(ocean_floor: &Floor) -> usize {
    ocean_floor.iter().flatten().filter(|&&x| x >= 2).count()
}
