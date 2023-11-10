#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance(&self, other_point: &Point) -> f64 {
        ((self.x - other_point.x).powf(2.0) + (self.y - other_point.y).powf(2.0)).sqrt()
    }
}

fn main() {
    let p1 = Point { x: 1.0, y: 1.0 };
    let p2 = Point { x: 2.0, y: 2.0 };
    println!("{}", p1.distance(&p2));
}
