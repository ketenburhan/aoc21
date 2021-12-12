const DATA: &str = include_str!("data.txt");

fn main() {
    let list: Vec<[&str; 2]> = DATA
        .lines()
        .map(|line| line.split('-').collect::<Vec<&str>>().try_into().unwrap())
        .collect::<Vec<[&str; 2]>>();
    println!("part1: {}", part1(&list, &["start"]));
    println!("part2: {}", part2(&list, &["start"], true));
}

fn part2(list: &[[&str; 2]], history: &[&str], small_cave_twice: bool) -> usize {
    let pos = history.last().unwrap();
    if *pos == "end" {
        return 1;
    }
    let neis = get_neighbours(list, pos);
    neis.into_iter()
        .filter(|nei| match history.iter().filter(|&x| *nei == *x).count() {
            0 => true,
            1 => {
                let first = nei.chars().next().unwrap();
                first.is_uppercase() || (small_cave_twice && first.is_lowercase())
            }
            _ => nei.chars().next().unwrap().is_uppercase(),
        })
        .filter(|nei| *nei != "start")
        .map(|nei| {
            let first = nei.chars().next().unwrap();
            let avail = small_cave_twice
                && (first.is_uppercase() || (first.is_lowercase() && !history.contains(&nei)));
            let mut new_history = history.to_vec();
            new_history.push(nei);
            part2(list, new_history.as_slice(), avail)
        })
        .sum()
}

fn part1(list: &[[&str; 2]], history: &[&str]) -> usize {
    let pos = history.last().unwrap();
    if *pos == "end" {
        return 1;
    }
    let neis = get_neighbours(list, pos);
    neis.into_iter()
        .filter(|nei| !history.contains(nei) || !nei.chars().next().unwrap().is_lowercase())
        .map(|nei| {
            let mut new_history = history.to_vec();
            new_history.push(nei);
            part1(list, new_history.as_slice())
        })
        .sum()
}

fn get_neighbours<'a>(list: &'a [[&str; 2]], cave: &'a str) -> Vec<&'a str> {
    list.iter()
        .filter_map(|x| {
            if x[0] == cave {
                Some(x[1])
            } else if x[1] == cave {
                Some(x[0])
            } else {
                None
            }
        })
        .collect()
}
