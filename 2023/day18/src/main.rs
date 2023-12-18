use aoc_utils::{TwoDimArray, Vector};
use regex::Regex;
use std::io::stdin;

struct Direction {
    dir: Vector,
    len: isize,
    color: String,
}

fn main() {
    let mut terrain: TwoDimArray<u8> = TwoDimArray::new(1, 1);
    let re = Regex::new(r"^([LRDU]) (\d+) \(#(.*?)\)$").unwrap();
    let mut dirs = vec![];
    for line in stdin().lines() {
        let line = line.unwrap();
        let caps = re.captures(&line).unwrap();
        let dir = &caps[1];
        let len: isize = (caps[2].parse::<isize>().unwrap());
        let color = &caps[3];

        let correct_len = isize::from_str_radix(&color[..5], 16).unwrap();
        let correct_dir = match &color[5..] {
            "0" => Vector::RIGHT,
            "1" => Vector::DOWN,
            "2" => Vector::LEFT,
            "3" => Vector::UP,
            _ => panic!()
        };

        let dir = match dir {
            "L" => Vector::LEFT,
            "R" => Vector::RIGHT,
            "U" => Vector::UP,
            "D" => Vector::DOWN,
            _ => panic!(),
        };
        dirs.push(Direction {
            dir: correct_dir,
            len: correct_len,
            color: color.to_string(),
        });
    }

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
            // tato pravotočivá, další pravotočivá
            point += j.dir * (j.len + 1);
        }
        if d1 > 0 && d2 <= 0 {
            // tato pravotočivá, další levotočivá
            point += j.dir * j.len;
        }
        if d1 <= 0 && d2 <= 0 {
            // tato levotočivá, další levotočivá
            point += j.dir * (j.len - 1);
        }
        if d1 <= 0 && d2 >= 0 {
            // tato levotočivá, další pravotočivá
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

    println!("{}", running_volume / 2);
}
