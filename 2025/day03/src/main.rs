use std::io::stdin;

fn main() {
    let mut silver = 0;
    for line in stdin().lines() {
        let line = line.unwrap();
        let bts = line.as_bytes();
        let (i, v) = bts[..bts.len() - 1]
            .iter()
            .enumerate()
            .max_by_key(|(_, val)| *val)
            .unwrap();
        let v = *v;
        let v2 = bts[i + 1..].iter().max().unwrap();
        let v2 = *v2;
        assert!(v.is_ascii_digit());
        assert!(v2.is_ascii_digit());
        let res = (v - b'0') as u32 * 10 + (v2 - b'0') as u32;
        println!("{}", res);
        silver += res;
    }
    println!("silver: {}", silver);
}
