use std::io::stdin;

fn get_initial(op: u8) -> u64 {
    match op {
        b'+' => 0,
        b'*' => 1,
        _ => unreachable!(),
    }
}

fn apply_op(current: u64, op: u8, num: u64) -> u64 {
    match op {
        b'+' => current + num,
        b'*' => current * num,
        _ => unreachable!(),
    }
}

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

        result += range.fold(get_initial(op), |acc, c| {
            let num = lines
                .iter()
                .take(lines.len() - 1)
                .map(|l| l.as_bytes()[c])
                .fold(0, |num, b| {
                    if b == b' ' {
                        num
                    } else {
                        10 * num + (b - b'0') as u64
                    }
                });
            if num == 0 {
                acc
            } else {
                apply_op(acc, op, num)
            }
        });
        start = i;
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
        let op = op.as_bytes()[0];
        silver += hw[0..hw.len() - 1]
            .iter()
            .map(|r| r[i])
            .fold(get_initial(op), |acc, c| {
                let num = c.parse::<u64>().unwrap();
                apply_op(acc, op, num)
            });
    }
    println!("silver: {}", silver);
    println!("gold: {}", gold(&lines));
}
