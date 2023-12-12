use std::io::stdin;

type N = i64;

fn print_dp(dp: &Vec<Vec<N>>) {
    for j in 0..dp.len() {
        eprint!("{:<3}", j);
        eprintln!(
            " {}",
            dp[j].iter().fold("".to_string(), |mut a, c| {
                a.push_str(&format!(" {}", c));
                a.to_string()
            })
        );
    }
}

fn rank_sequence(s: &[u8]) -> Vec<N> {
    let mut current_sequence: N = 0;
    let mut ranking = vec![];
    for &c in s {
        match c {
            b'.' => {
                if current_sequence != 0 {
                    ranking.push(current_sequence);
                    current_sequence = 0;
                }
            }
            b'#' => current_sequence += 1,
            _ => panic!(),
        }
    }
    if current_sequence != 0 {
        ranking.push(current_sequence);
    }
    ranking
}

fn count_possibilities(spring_conditions: &[u8], groups: &[N]) -> usize {
    let mut all_possibilities = match spring_conditions[0] {
        b'.' => vec![".".to_string()],
        b'#' => vec!["#".to_string()],
        b'?' => vec![".".to_string(), "#".to_string()],
        _ => panic!(),
    };
    for &c in &spring_conditions[1..] {
        for i in 0..all_possibilities.len() {
            match c {
                b'.' => all_possibilities[i].push('.'),
                b'#' => all_possibilities[i].push('#'),
                b'?' => {
                    let j = all_possibilities.len();
                    all_possibilities.push(all_possibilities[i].clone());
                    all_possibilities[i].push('#');
                    all_possibilities[j].push('.');
                }
                _ => panic!(),
            }
        }
    }
    all_possibilities
        .iter()
        .filter(|x| rank_sequence(x.as_bytes()) == groups)
        .count()
}

fn main() {
    let mut silver = 0;
    for line in stdin().lines() {
        let line = line.unwrap();
        let (spring_conditions, groups) = line.split_once(' ').unwrap();
        let groups: Vec<N> = groups.split(',').map(|x| x.parse().unwrap()).collect();
        let spring_conditions = spring_conditions.as_bytes();
        // let mut dp = vec![vec![0; groups.len() + 1]; map_row.len() + 1];

        silver += count_possibilities(spring_conditions, &groups);
    }
    println!("{}", silver);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        assert_eq!(&crate::rank_sequence("###.##.#".as_bytes()), &[3, 2, 1]);
        assert_eq!(&crate::rank_sequence("#.#.###".as_bytes()), &[1, 1, 3]);
    }
}
