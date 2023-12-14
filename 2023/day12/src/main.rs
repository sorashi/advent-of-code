use std::io::stdin;
use std::iter::repeat;

type N = usize;

fn count_possibilities(spring_conditions: &[u8], groups: &[N]) -> usize {
    let max_k = *groups.iter().max().unwrap();
    // dp[x][y][k] is the count of arrangements in spring_conditions[..=x] using only groups[..=y] while the sequence ends with a consecutive run of k #'s
    // To avoid edge cases, we add a last group with 0 and a last char dot.
    let mut groups = Vec::from(groups);
    groups.push(0);
    let mut spring_conditions = Vec::from(spring_conditions);
    spring_conditions.push(b'.');

    let mut dp = vec![vec![vec![0; max_k + 1]; groups.len()]; spring_conditions.len()];

    // base cases
    match spring_conditions[0] {
        b'#' => dp[0][0][1] = 1,
        b'.' => dp[0][0][0] = 1,
        b'?' => {
            dp[0][0][0] = 1;
            dp[0][0][1] = 1;
        }
        _ => panic!()
    }

    // build the table
    for x in 1..spring_conditions.len() {
        let c = spring_conditions[x];
        for y in 0..groups.len() {
            for k in 0..=groups[y] {
                // count for if the current char is a dot
                let mut dot = 0;
                if k == 0 { // dot makes sense only if current consecutive run is 0
                    if y > 0 {
                        dot = dp[x - 1][y - 1][groups[y - 1]] + dp[x - 1][y][0];
                    } else if !spring_conditions[..=x].iter().any(|x| *x == b'#') {
                        // k == 0 && y == 0, so all chars up to x must be .'s (including ?'s)
                        // (that is the only possible arrangement)
                        dot = 1;
                    }
                }
                let mut hash = 0;
                if k != 0 {
                    hash = dp[x - 1][y][k - 1];
                }
                dp[x][y][k] = match c {
                    b'.' => dot,
                    b'#' => hash,
                    b'?' => dot + hash,
                    _ => panic!(),
                }
            }
        }
    }
    dp[spring_conditions.len() - 1][groups.len() - 1][0]
}

fn main() {
    let mut silver = 0;
    let mut gold = 0;
    for line in stdin().lines() {
        let line = line.unwrap();
        let (spring_conditions, groups) = line.split_once(' ').unwrap();
        let groups: Vec<N> = groups.split(',').map(|x| x.parse().unwrap()).collect();
        let spring_conditions = spring_conditions.as_bytes();
        silver += count_possibilities(spring_conditions, &groups);

        let spring_conditions = repeat(spring_conditions)
            .take(5)
            .collect::<Vec<_>>()
            .join(&b'?');
        let groups: Vec<_> = repeat(groups).take(5).flatten().collect();
        gold += count_possibilities(&spring_conditions, &groups);
    }
    println!("{}", silver);
    println!("{}", gold);
}
