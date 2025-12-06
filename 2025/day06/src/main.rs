use std::io::stdin;

fn main() {
    let lines = stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    let mut hw = vec![];
    for line in &lines {
        let x = line.split_whitespace().collect::<Vec<_>>();
        hw.push(x);
    }
    let mut silver = 0;
    for (i, op) in hw.last().unwrap().iter().enumerate() {
        let mut res = match *op {
            "+" => 0,
            "*" => 1,
            _ => unreachable!(),
        };
        for num in hw[0..hw.len() - 1].iter().map(|r| r[i]) {
            let num = num.parse::<u64>().unwrap();
            res = match *op {
                "+" => res + num,
                "*" => res * num,
                _ => unreachable!(),
            };
        }
        silver += res;
    }
    println!("silver: {}", silver);
}
