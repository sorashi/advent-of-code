use std::io::stdin;

type N = i64;

fn main() {
    let mut silver: N = 0;
    let mut gold: N = 0;
    for line in stdin().lines() {
        let line = line.unwrap();
        let nums: Vec<N> = line.split(' ').map(|x| x.parse().unwrap()).collect();
        let mut numnums: Vec<Vec<N>> = vec![nums];
        loop {
            let diffs: Vec<N> = numnums
                .last()
                .unwrap()
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect();

            if diffs.iter().all(|x| *x == 0) {
                numnums.push(diffs);
                break;
            }
            numnums.push(diffs);
        }
        let mut current_silver = 0;
        let mut current_gold = 0;
        for i in (0..numnums.len() - 1).rev() {
            current_silver += numnums[i].last().unwrap();
            current_gold = numnums[i][0] - current_gold;
        }
        silver += current_silver;
        gold += current_gold;
    }
    println!("silver: {:?}", silver);
    println!("gold: {:?}", gold);
}
