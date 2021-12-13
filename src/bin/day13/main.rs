const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part1: {}", part1(DATA));
    println!("part2:");
    part2(DATA);
}

fn part1(data: &str) -> usize {
    let data: Vec<&str> = data.split("\n\n").collect();
    let mut dot_list: Vec<[u32; 2]> = data[0]
        .lines()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<u32>>()
                .try_into()
                .unwrap()
        })
        .collect();

    // x = 0
    // y = 1
    let fold_list: Vec<(u8, u32)> = data[1]
        .lines()
        .map(|line| line.split(' ').nth(2).unwrap())
        .map(|eq| {
            let mut splitted = eq.split('=');
            (
                if splitted.next().unwrap() == "x" {
                    0
                } else {
                    1
                },
                splitted.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let (axis, i) = fold_list[0];
    let axis = axis as usize;
    dot_list.iter_mut().for_each(|coords| {
        if coords[axis] > i {
            coords[axis] -= (coords[axis] - i) * 2;
        }
    });

    // make unique
    for x in (0..dot_list.len()).rev() {
        for y in (x + 1..dot_list.len()).rev() {
            if dot_list[x] == dot_list[y] {
                dot_list.remove(y);
            }
        }
    }
    dot_list.len()
}
fn part2(data: &str) {
    let data: Vec<&str> = data.split("\n\n").collect();
    let mut dot_list: Vec<[u32; 2]> = data[0]
        .lines()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<u32>>()
                .try_into()
                .unwrap()
        })
        .collect();

    // x = 0
    // y = 1
    let fold_list: Vec<(u8, u32)> = data[1]
        .lines()
        .map(|line| line.split(' ').nth(2).unwrap())
        .map(|eq| {
            let mut splitted = eq.split('=');
            (
                if splitted.next().unwrap() == "x" {
                    0
                } else {
                    1
                },
                splitted.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    for (axis, i) in fold_list {
        let axis = axis as usize;
        dot_list.iter_mut().for_each(|coords| {
            if coords[axis] > i {
                coords[axis] -= (coords[axis] - i) * 2;
            }
        });

        // make unique
        for x in (0..dot_list.len()).rev() {
            for y in (x + 1..dot_list.len()).rev() {
                if dot_list[x] == dot_list[y] {
                    dot_list.remove(y);
                }
            }
        }
    }

    let mut map = [['.'; 41]; 7];
    for dot in dot_list {
        map[dot[1] as usize][dot[0] as usize] = '#';
    }

    for line in map {
        for val in line {
            print!("{} ", val);
        }
        println!();
    }
}
