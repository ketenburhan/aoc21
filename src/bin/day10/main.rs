const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part1: {}", part1(DATA));
    println!("part2: {}", part2(DATA));
}

fn part1(data: &str) -> usize {
    data.lines()
        .map(|line| {
            let mut openings = vec![];
            let mut illegal = None;
            for c in line.chars() {
                match c {
                    '(' | '[' | '{' | '<' => openings.push(c),
                    ')' => {
                        let o = openings.pop().unwrap();
                        if o != '(' {
                            illegal = Some(c);
                            break;
                        }
                    }
                    ']' => {
                        let o = openings.pop().unwrap();
                        if o != '[' {
                            illegal = Some(c);
                            break;
                        }
                    }
                    '}' => {
                        let o = openings.pop().unwrap();
                        if o != '{' {
                            illegal = Some(c);
                            break;
                        }
                    }
                    '>' => {
                        let o = openings.pop().unwrap();
                        if o != '<' {
                            illegal = Some(c);
                            break;
                        }
                    }
                    _ => unreachable!(),
                }
            }

            match illegal {
                Some(')') => 3,
                Some(']') => 57,
                Some('}') => 1197,
                Some('>') => 25137,
                _ => 0,
            }
        })
        .sum()
}

fn part2(data: &str) -> usize {
    let mut total_scores: Vec<usize> = data
        .lines()
        .map(|line| {
            let mut openings = vec![];
            for c in line.chars() {
                match c {
                    '(' | '[' | '{' | '<' => openings.push(c),
                    ')' => {
                        let o = openings.pop().unwrap();
                        if o != '(' {
                            openings.push(o);
                            return None;
                        }
                    }
                    ']' => {
                        let o = openings.pop().unwrap();
                        if o != '[' {
                            openings.push(o);
                            return None;
                        }
                    }
                    '}' => {
                        let o = openings.pop().unwrap();
                        if o != '{' {
                            openings.push(o);
                            return None;
                        }
                    }
                    '>' => {
                        let o = openings.pop().unwrap();
                        if o != '<' {
                            openings.push(o);
                            return None;
                        }
                    }
                    _ => unreachable!(),
                }
            }

            Some(openings)
        })
        .filter_map(|openings_option| {
            if let Some(openings) = openings_option {
                let mut total_score: usize = 0;
                openings
                    .iter()
                    .rev()
                    .map(|c| match c {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => unreachable!(),
                    })
                    .for_each(|score| {
                        total_score *= 5;
                        total_score += score;
                    });
                Some(total_score)
            } else {
                None
            }
        })
        .collect();
    total_scores.sort_unstable();
    total_scores[total_scores.len() / 2]
}
