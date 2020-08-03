// find the distance
trait Distance{
    fn distance (&self, p: &Self) -> f64;
}

// Two-dimensional point
struct Point{
    x: f64, y: f64
}

impl Point{
    fn new(x1: f64, y1: f64) -> Point{
        Point{x: x1, y: y1}
    }
}
