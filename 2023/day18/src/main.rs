use aoc_utils::Vector;
use std::io::stdin;

struct Direction {
    dir: Vector,
    len: isize,
}

fn get_volume(dirs: &[Direction]) -> isize {
    let mut points = vec![];
    let mut point = dirs[0].dir * (dirs[0].len + 1);
    points.push(Vector::ZERO);
    points.push(point);
    for windows in dirs.windows(2).collect::<Vec<_>>().windows(2) {
        let window1 = windows[0];
        let window2 = windows[1];
        let i = &window1[0];
        let j = &window1[1];
        let k = &window2[0];
        let l = &window2[1];
        let d1 = i.dir.x * j.dir.y - j.dir.x * i.dir.y;
        let d2 = k.dir.x * l.dir.y - l.dir.x * k.dir.y;
        if d1 > 0 && d2 > 0 {
            // window1 is clockwise, window2 is clockwise angle
            point += j.dir * (j.len + 1);
        }
        if d1 > 0 && d2 <= 0 {
            // window1 is clockwise, window2 is counter-clockwise
            point += j.dir * j.len;
        }
        if d1 <= 0 && d2 <= 0 {
            // window1 is counter-clockwise, window2 is counter-clockwise
            point += j.dir * (j.len - 1);
        }
        if d1 <= 0 && d2 >= 0 {
            // window1 is counter-clockwise, window2 is clockwise
            point += j.dir * j.len;
        }

        points.push(point);
    }

    // shoelace formula
    let mut running_volume = 0;
    for window in points.windows(2) {
        let p1 = window[0];
        let p2 = window[1];
        running_volume += (p1.y + p2.y) * (p1.x - p2.x);
    }
    running_volume / 2
}

fn main() {
    let mut silver_dirs = vec![];
    let mut gold_dirs = vec![];
    for line in stdin().lines() {
        let line = line.unwrap();
        let split: Vec<_> = line.split(' ').collect();
        let silver_dir = split[0];
        let silver_len: isize = split[1].parse().unwrap();
        let color = &split[2][2..8];

        let silver_dir = match silver_dir {
            "L" => Vector::LEFT,
            "R" => Vector::RIGHT,
            "U" => Vector::UP,
            "D" => Vector::DOWN,
            _ => panic!(),
        };

        let gold_len = isize::from_str_radix(&color[..5], 16).unwrap();
        let gold_dir = match &color[5..] {
            "0" => Vector::RIGHT,
            "1" => Vector::DOWN,
            "2" => Vector::LEFT,
            "3" => Vector::UP,
            _ => panic!(),
        };

        silver_dirs.push(Direction {
            dir: silver_dir,
            len: silver_len,
        });
        gold_dirs.push(Direction {
            dir: gold_dir,
            len: gold_len,
        });
    }

    println!("silver: {}", get_volume(&silver_dirs));
    println!("gold: {}", get_volume(&gold_dirs));
}
