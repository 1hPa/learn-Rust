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

// Implementation
impl Distance for Point{
    fn distance (&self, p: &Point) -> f64{
        let dx = self.x - p.x;
        let dy = self.y - p.y;
        (dx * dx + dy * dy).sqrt()
    }
}

// three dimensional point
struct Point{
    x: f64, y: f64, z: f64
}

impl Point3D{
    fn new(x1: f64, y1: f64, z1: f64) -> Point{
        Point{x: x1, y: y1, z: z1}
    }
}

// Implementation(three-dismensional)
impl Distance for Point3D{
    fn distance (&self, p: &Point) -> f64{
        let dx = self.x - p.x;
        let dy = self.y - p.y;
        (dx * dx * dy * dy).sqrt()
    }
}

fn main(){
    let p1 = Point::new(0.0,0.0);
    let p2 = Point::new(10.0, 10.0);
    println!("{}", p1.distance(&p2));
}
