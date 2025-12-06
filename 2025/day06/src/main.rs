use std::io::stdin;

fn gold(lines: &[String]) -> u64 {
    let ops = lines.last().unwrap().as_bytes();
    let mut result = 0;
    let mut start = 0;
    while start < ops.len() {
        let mut i = start + 1;
        while i < ops.len() && ops[i] == b' ' {
            i += 1;
        }
        let range = start..i;
        let op = ops[range.start];

        let mut local_result = match op {
            b'+' => 0,
            b'*' => 1,
            _ => unreachable!(),
        };
        for i in range {
            let mut num = 0;
            for j in 0..lines.len() - 1 {
                let b = lines[j].as_bytes()[i];
                if b == b' ' {
                    continue;
                }
                num = 10 * num + (b - b'0') as u64;
            }
            if num == 0 {
                continue;
            }
            local_result = match op {
                b'+' => local_result + num,
                b'*' => local_result * num,
                _ => unreachable!(),
            };
        }
        start = i;
        result += local_result;
    }
    result
}

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
    println!("gold: {}", gold(&lines));
}
