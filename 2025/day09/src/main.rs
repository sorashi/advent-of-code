use std::io::stdin;

#[derive(Copy, Clone)]
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
fn line_intersects_rect_interior(p1: &Vector, p2: &Vector, r1: &Vector, r2: &Vector) -> bool {
    let left = r1.x.min(r2.x);
    let right = r1.x.max(r2.x);
    let top = r1.y.min(r2.y);
    let bottom = r1.y.max(r2.y);
    if p1.x == p2.x {
        // vertical line
        // must be strictly inside left and right
        if p1.x > left && p1.x < right {
            // y range must overlap
            let line_top = p1.y.min(p2.y);
            let line_bottom = p1.y.max(p2.y);
            let overlap_top = line_top.max(top);
            let overlap_bottom = line_bottom.min(bottom);
            if overlap_top < overlap_bottom {
                return true;
            }
        }
    } else if p1.y == p2.y {
        // horizontal line
        // must be strictly inside top and bottom
        if p1.y > top && p1.y < bottom {
            // x range must overlap
            let line_left = p1.x.min(p2.x);
            let line_right = p1.x.max(p2.x);
            let overlap_left = line_left.max(left);
            let overlap_right = line_right.min(right);
            if overlap_left < overlap_right {
                return true;
            }
        }
    } else {
        panic!("line is not axis-aligned");
    }
    false
}
fn is_point_in_rectilinear(px: f64, py: f64, poly: &[Vector]) -> bool {
    let mut inside = false;
    for i in 0..poly.len() {
        let v1 = &poly[i];
        let v2 = &poly[(i + 1) % poly.len()];
        if v1.x == v2.x {
            // vertical line
            let line_top = v1.y.min(v2.y) as f64;
            let line_bottom = v1.y.max(v2.y) as f64;
            if line_top < py && py < line_bottom && (v1.x as f64) > px {
                inside = !inside;
            }
        }
    }
    inside
}
fn is_rect_in_rectilinear(r1: &Vector, r2: &Vector, poly: &[Vector]) -> bool {
    let left = r1.x.min(r2.x);
    let right = r1.x.max(r2.x);
    let top = r1.y.min(r2.y);
    let bottom = r1.y.max(r2.y);
    let mid_x = (left + right) as f64 / 2.0;
    let mid_y = (top + bottom) as f64 / 2.0;
    if !is_point_in_rectilinear(mid_x, mid_y, poly) {
        return false;
    }
    for i in 0..poly.len() {
        let p1 = &poly[i];
        let p2 = &poly[(i + 1) % poly.len()];
        if line_intersects_rect_interior(p1, p2, r1, r2) {
            return false;
        }
    }
    true
}

fn main() {
    let mut v = vec![];
    for line in stdin().lines() {
        let line = line.unwrap();
        let (x, y) = line.split_once(',').unwrap();
        let x = x.parse().unwrap();
        let y = y.parse().unwrap();
        v.push(Vector::new(x, y));
    }
    let mut silver = 0;
    let mut gold = 0;
    for i in 0..v.len() - 1 {
        for j in i + 1..v.len() {
            let area = v[i].area_between(&v[j]);
            silver = silver.max(area);

            let inside = is_rect_in_rectilinear(&v[i], &v[j], &v);
            if inside {
                gold = gold.max(area);
            }
        }
    }
    println!("silver: {}", silver);
    println!("gold: {}", gold);
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
