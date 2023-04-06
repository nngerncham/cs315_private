pub type Point = (i32, i32);
pub type Distance = i64;

pub fn dist(p1: &Point, p2: &Point) -> Distance {
    let (x1, y1) = p1;
    let (x2, y2) = p2;

    let dx = x1 - x2;
    let dy = y1 - y2;

    (dx.pow(2) + dy.pow(2)) as Distance
}

pub fn pt_default() -> Point {
    (0, 0)
}
