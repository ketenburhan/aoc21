const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part1: {}", part1(DATA));
    println!("part2: {}", part2(DATA));
}

fn part1(data: &str) -> usize {
    let mut flashes = 0;
    let mut map: [[u8; 10]; 10] = data
        .lines()
        .map(|line| {
            line.bytes()
                .map(|c| {
                    c - 48 // ascii numbers starts at 48 (0: 48, 1: 49, etc.)
                })
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<[u8; 10]>>()
        .try_into()
        .unwrap();

    for _ in 0..100 {
        #[allow(clippy::needless_range_loop)]
        for i in 0..10 {
            for j in 0..10 {
                map[i][j] += 1;
            }
        }
        for i in 0..10 {
            for j in 0..10 {
                if map[i][j] == 10 {
                    flashes += 1;
                    map[i][j] = 255;
                    update_adjents(&mut map, &mut flashes, i, j);
                }
            }
        }

        #[allow(clippy::needless_range_loop)]
        for i in 0..10 {
            for j in 0..10 {
                if map[i][j] == 255 {
                    map[i][j] = 0;
                }
            }
        }
    }
    flashes
}

fn part2(data: &str) -> usize {
    let mut map: [[u8; 10]; 10] = data
        .lines()
        .map(|line| {
            line.bytes()
                .map(|c| {
                    c - 48 // ascii numbers starts at 48 (0: 48, 1: 49, etc.)
                })
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<[u8; 10]>>()
        .try_into()
        .unwrap();

    let mut step_count = 0;

    loop {
        step_count += 1;
        let mut flashes = 0;

        #[allow(clippy::needless_range_loop)]
        for i in 0..10 {
            for j in 0..10 {
                map[i][j] += 1;
            }
        }
        for i in 0..10 {
            for j in 0..10 {
                if map[i][j] == 10 {
                    flashes += 1;
                    map[i][j] = 255;
                    update_adjents(&mut map, &mut flashes, i, j);
                }
            }
        }
        if flashes == 100 {
            break;
        }

        #[allow(clippy::needless_range_loop)]
        for i in 0..10 {
            for j in 0..10 {
                if map[i][j] == 255 {
                    map[i][j] = 0;
                }
            }
        }
    }
    step_count
}

fn update_adjents(map: &mut [[u8; 10]; 10], flashes: &mut usize, i: usize, j: usize) {
    // is available
    #[rustfmt::skip]
    let (left, right, top, bottom) = (
        j != 0,
        j != 9,
        i != 0,
        i != 9
    );

    if top {
        update_position(map, flashes, i - 1, j);
    }
    if right {
        update_position(map, flashes, i, j + 1);
    }
    if bottom {
        update_position(map, flashes, i + 1, j);
    }
    if left {
        update_position(map, flashes, i, j - 1);
    }

    if right && top {
        update_position(map, flashes, i - 1, j + 1);
    }
    if right && bottom {
        update_position(map, flashes, i + 1, j + 1);
    }
    if left && bottom {
        update_position(map, flashes, i + 1, j - 1);
    }
    if left && top {
        update_position(map, flashes, i - 1, j - 1);
    }
}
fn update_position(map: &mut [[u8; 10]; 10], flashes: &mut usize, i: usize, j: usize) {
    if map[i][j] != 255 {
        map[i][j] += 1;

        if map[i][j] >= 10 {
            *flashes += 1;
            map[i][j] = 255;
            update_adjents(map, flashes, i, j);
        }
    }
}
