use std::io::stdin;

struct Vector {
    x: usize,
    y: usize,
}
impl Vector {
    fn new(x: usize, y: usize) -> Self {
        Vector { x, y }
    }
    fn area_between(&self, other: &Vector) -> usize {
        let width = self.x.abs_diff(other.x) + 1;
        let height = self.y.abs_diff(other.y) + 1;
        width * height
    }
}

fn main() {
    let mut v = vec![];
    for line in stdin().lines() {
        let line = line.unwrap();
        let (x, y) = line.split_once(',').unwrap();
        let x = x.parse::<usize>().unwrap();
        let y = y.parse::<usize>().unwrap();
        v.push(Vector::new(x, y));
    }
    let mut silver = 0;
    for i in 0..v.len() - 1 {
        for j in i + 1..v.len() {
            let area = v[i].area_between(&v[j]);
            silver = silver.max(area);
        }
    }
    println!("silver: {}", silver);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_between() {
        let v1 = Vector::new(2, 5);
        let v2 = Vector::new(9, 7);
        assert_eq!(v1.area_between(&v2), 24);

        let v1 = Vector::new(7, 1);
        let v2 = Vector::new(11, 7);
        assert_eq!(v1.area_between(&v2), 35);

        let v1 = Vector::new(7, 3);
        let v2 = Vector::new(2, 3);
        assert_eq!(v1.area_between(&v2), 6);

        let v1 = Vector::new(2, 5);
        let v2 = Vector::new(11, 1);
        assert_eq!(v1.area_between(&v2), 50);
    }
}
