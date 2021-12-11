const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part1: {}", part1(DATA));
    println!("part2: {}", part2(DATA));
}

fn part1(data: &str) -> usize {
    let mut map = [[9_usize; 102]; 102];
    for (i, line) in data.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            map[i + 1][j + 1] = c.to_string().parse().unwrap();
        }
    }
    let mut low_points = vec![];

    for i in 1..map.len() - 1 {
        for j in 1..map[0].len() - 1 {
            let now = map[i][j];
            if now < map[i - 1][j]
                && now < map[i][j + 1]
                && now < map[i + 1][j]
                && now < map[i][j - 1]
            {
                low_points.push(now);
            }
        }
    }
    low_points.len() + low_points.iter().sum::<usize>()
}

fn part2(data: &str) -> usize {
    let mut map = [[9_usize; 102]; 102];
    for (i, line) in data.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            map[i + 1][j + 1] = c.to_string().parse().unwrap();
        }
    }
    let mut basins = vec![];

    for i in 1..map.len() - 1 {
        for j in 1..map[0].len() - 1 {
            let now = map[i][j];
            if now < map[i - 1][j]
                && now < map[i][j + 1]
                && now < map[i + 1][j]
                && now < map[i][j - 1]
            {
                println!("{} {} {}", j, i, now);
                let mut new_map = map;
                new_map[i][j] = 10 + now;
                basins.push(basin_len(&mut new_map, j, i));
            }
        }
    }
    basins.sort_unstable();
    basins
        .iter()
        .rev()
        .inspect(|x| println!("{}", x))
        .take(3)
        .product()
}

fn basin_len(map: &mut [[usize; 102]; 102], j: usize, i: usize) -> usize {
    let mut out = 1;
    let dirs = [(j + 1, i), (j, i + 1), (j - 1, i), (j, i - 1)];
    for dir in dirs {
        let val = map[dir.1][dir.0];
        if val < 9 {
            map[dir.1][dir.0] = val + 10;
            out += basin_len(map, dir.0, dir.1);
        }
    }
    out
}
