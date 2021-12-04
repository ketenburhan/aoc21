use std::fs;

#[derive(Clone, Debug)]
struct Board([i8; 25], bool);
impl Board {
    fn rotate(&self) -> [i8; 25] {
        let mut out = [0; 25];
        for col_index in 0..5 {
            for row_index in 0..5 {
                out[col_index * 5 + row_index] = self.0[row_index * 5 + col_index];
            }
        }
        out
    }
    fn apply_number(&mut self, number: i8) {
        self.0.iter_mut().find_map(|x| {
            if number == *x {
                *x = -x.to_owned();
                Some(number)
            } else {
                None
            }
        });
    }
    fn check_win(&self) -> bool {
        let rotated = self.rotate();

        self.0.chunks(5).any(|row| row.iter().all(|&x| x < 0))
            || rotated.chunks(5).any(|col| col.iter().all(|&x| x < 0))
    }
    fn get_unmarked_sum(&self) -> i64 {
        self.0
            .iter()
            .map(|&x| if x > 0 { x as i64 } else { 0 })
            .sum()
    }
}
impl TryInto<Board> for Vec<i8> {
    type Error = Vec<i8>;

    fn try_into(self) -> Result<Board, Self::Error> {
        let a = self.try_into()?;
        Ok(Board(a, false))
    }
}
fn main() {
    let contents =
        fs::read_to_string("./inputs/day4.txt").expect("Something went wrong reading the file");

    let mut splitted = contents.split("\n\n");

    let numbers: Vec<i8> = splitted
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut boards: Vec<Board> = splitted
        .map(|board_str| {
            board_str
                .split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i8>>()
                .try_into()
                .unwrap()
        })
        .collect();

    let mut results = vec![];
    for &number in numbers.iter() {
        boards.iter_mut().for_each(|board| {
            // is board finished
            if board.1 {
                return;
            }
            board.apply_number(number);
            if board.check_win() {
                results.push((number, board.to_owned()));
                board.1 = true;
            }
        })
    }

    println!("{}", part1(&results));
    println!("{}", part2(&results));
}

fn part1(results: &[(i8, Board)]) -> i64 {
    let first = results.first().unwrap();
    let unmarked_sum = first.1.get_unmarked_sum();

    unmarked_sum * first.0 as i64
}

fn part2(results: &[(i8, Board)]) -> i64 {
    let last = results.last().unwrap();
    let unmarked_sum = last.1.get_unmarked_sum();

    unmarked_sum * last.0 as i64
}
