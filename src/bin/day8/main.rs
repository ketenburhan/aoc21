const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part1: {}", part1(DATA));
    println!("part2: {}", part2(DATA));
}

fn part1(data: &str) -> usize {
    data.split(&['|', '\n'][..])
        .skip(1)
        .step_by(2)
        .map(|x| {
            x.split(' ')
                .filter(|x| matches!(x.len(), 2 | 3 | 4 | 7))
                .count()
        })
        .sum()
}
fn part2(data: &str) -> i32 {
    data.lines()
        .map(|line| {
            let mut in_out = line
                .split('|')
                .map(|x| x.split(' ').map(|x| x.chars().collect()).collect());
            let inputs: Vec<Vec<char>> = in_out.next().unwrap();
            let outputs: Vec<Vec<char>> = in_out.next().unwrap();

            let segments = get_segments(inputs);

            outputs
                .iter()
                .skip(1) // skip single ' '
                .zip((0..=3).rev())
                .map(|(v, pow)| {
                    decode_digit(v.clone().as_mut_slice(), &segments) as i32 * 10_i32.pow(pow)
                })
                .sum::<i32>()
        })
        .sum()
}
fn get_segments(inputs: Vec<Vec<char>>) -> [char; 7] {
    let s1 = inputs.iter().find(|x| x.len() == 2).unwrap();
    let s7 = inputs.iter().find(|x| x.len() == 3).unwrap();
    let s4 = inputs.iter().find(|x| x.len() == 4).unwrap();
    let s8 = inputs.iter().find(|x| x.len() == 7).unwrap();
    let s069: Vec<&Vec<char>> = inputs.iter().filter(|x| x.len() == 6).collect();
    let s235: Vec<&Vec<char>> = inputs.iter().filter(|x| x.len() == 5).collect();

    let seg_a = complement(s7, s1)[0];
    let (s6, seg_c, seg_f) = s069
        .iter()
        .find_map(|&v| {
            let inter = intersection(v, s1);
            if inter.len() == 1 {
                Some((
                    v,
                    *s1.iter().find(|&&x| x != inter[0]).unwrap(),
                    *s1.iter().find(|&&x| x == inter[0]).unwrap(),
                ))
            } else {
                None
            }
        })
        .unwrap();
    let s09: Vec<&Vec<char>> = s069.iter().filter(|&&x| x != s6).copied().collect();
    let s0 = s09.iter().find(|&x| union(x, s4).len() == 7).unwrap();
    let s9 = s09.iter().find(|&x| union(x, s4).len() != 7).unwrap();
    let seg_d = complement(s8, s0)[0];
    let seg_e = complement(s8, s9)[0];
    let horizontal_segs = intersection(&intersection(s235[0], s235[1]), s235[2]);
    let seg_g = complement(&horizontal_segs, &[seg_a, seg_d])[0];
    let seg_b = complement(s4, &[seg_c, seg_d, seg_f])[0];
    [seg_a, seg_b, seg_c, seg_d, seg_e, seg_f, seg_g]
}
#[allow(clippy::many_single_char_names)]
fn decode_digit(code: &mut [char], segments: &[char; 7]) -> usize {
    let [a, b, c, d, e, f, g] = *segments;
    let mut s = [
        vec![a, b, c, e, f, g],
        vec![c, f],
        vec![a, c, d, e, g],
        vec![a, c, d, f, g],
        vec![b, c, d, f],
        vec![a, b, d, f, g],
        vec![a, b, d, e, f, g],
        vec![a, c, f],
        vec![a, b, c, d, e, f, g],
        vec![a, b, c, d, f, g],
    ];

    s.iter_mut().for_each(|x| x.sort_unstable());
    code.sort_unstable();

    s.iter()
        .enumerate()
        .find(|(_, x)| **x == code.to_vec())
        .unwrap()
        .0
}

fn union(v1: &[char], v2: &[char]) -> Vec<char> {
    let mut out = v1.to_vec();
    for c in v2 {
        if !v1.contains(c) {
            out.push(*c);
        }
    }
    out
}

fn intersection(v1: &[char], v2: &[char]) -> Vec<char> {
    v1.iter().filter(|&c| v2.contains(c)).copied().collect()
}

fn complement(v1: &[char], v2: &[char]) -> Vec<char> {
    v1.iter().filter(|&c| !v2.contains(c)).copied().collect()
}
