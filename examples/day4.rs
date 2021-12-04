use std::fs;
use std::str::FromStr;

#[derive(Clone, Debug)]
struct GameState {
    numbers: Vec<u8>,
    boards: Vec<Board>,
}
impl FromStr for GameState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.split("\n\n");
        let numbers: Vec<u8> = splitted
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        let boards: Vec<Board> = splitted
            .map(|board_str| {
                let board_vec = board_str
                    .lines()
                    .map(|line| {
                        line.split_ascii_whitespace()
                            .filter_map(|s| s.parse::<i8>().ok())
                            .collect()
                    })
                    .collect();
                Board(board_vec)
            })
            .collect();
        Ok(Self { numbers, boards })
    }
}
#[derive(Clone, Debug)]
struct Board(Vec<Vec<i8>>);
impl Board {
    fn column_iter(&self, col_index: usize) -> BoardColumnIter {
        BoardColumnIter {
            board: self,
            col_index,
            row_index: 0,
        }
    }
}
struct BoardColumnIter<'a> {
    board: &'a Board,
    col_index: usize,
    row_index: usize,
}
impl<'a> Iterator for BoardColumnIter<'a> {
    type Item = i8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row_index >= 5 {
            None
        } else {
            let val = self.board.0[self.row_index][self.col_index];
            self.row_index += 1;
            Some(val)
        }
    }
}
impl Board {
    #[allow(clippy::ptr_arg, dead_code)]
    fn check_win(&self) -> bool {
        let mut vertical = false;
        for col_index in 0..5 {
            if self.column_iter(col_index).all(|x| x < 0) {
                vertical = true;
                break;
            }
        }
        // horizontal
        self.0.iter().any(|row| row.iter().all(|&x| x < 0)) || vertical
    }
    fn apply_number(&mut self, number: u8) {
        'out: for row in self.0.iter_mut() {
            for num in row.iter_mut() {
                if *num == number as i8 {
                    *num = -*num;
                    break 'out;
                }
            }
        }
    }
    fn get_unmarked_sum(&self) -> i32 {
        let filtered = self
            .0
            .iter()
            .flat_map(|x| x.clone().iter().map(|&x| x as i32).collect::<Vec<i32>>())
            .filter(|&x| x > 0);
        println!("filtered: {:#?}", filtered.clone().collect::<Vec<i32>>());
        filtered.reduce(|x, y| x + y).unwrap()
    }
}

fn main() {
    let contents =
        fs::read_to_string("./inputs/day4.txt").expect("Something went wrong reading the file");

    let game_state = GameState::from_str(&contents).unwrap();

    println!("{}", part1(game_state.clone()));
    println!("{}", part2(game_state));
}

fn part1(game_state: GameState) -> i64 {
    let results: Vec<Option<(Board, usize, u8)>> = game_state
        .boards
        .clone()
        .iter_mut()
        .map(|board| {
            for (i, &number) in game_state.numbers.iter().enumerate() {
                board.apply_number(number);
                if board.check_win() {
                    return Some((board.clone(), i, number));
                }
            }
            None
        })
        .collect();
    let min = results
        .iter()
        .filter_map(|x| x.as_ref())
        .min_by_key(|x| x.1)
        .unwrap();
    let unmarked_sum = min.0.get_unmarked_sum();

    unmarked_sum as i64 * min.2 as i64
}

fn part2(game_state: GameState) -> i64 {
    let results: Vec<Option<(Board, usize, u8)>> = game_state
        .boards
        .clone()
        .iter_mut()
        .map(|board| {
            for (i, &number) in game_state.numbers.iter().enumerate() {
                board.apply_number(number);
                if board.check_win() {
                    return Some((board.clone(), i, number));
                }
            }
            None
        })
        .collect();
    let max = results
        .iter()
        .filter_map(|x| x.as_ref())
        .max_by_key(|x| x.1)
        .unwrap();
    let unmarked_sum = max.0.get_unmarked_sum();

    unmarked_sum as i64 * max.2 as i64
}
