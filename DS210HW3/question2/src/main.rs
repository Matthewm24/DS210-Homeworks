use std::f64::consts::PI;

struct Polygon {
    side: f64,
    side_count: f64,
}

trait Shape {
    fn perimeter(&self) -> f64;
    fn area(&self) -> f64;
    fn radius(&self) -> f64;
    fn apothem(&self) -> f64;
    fn inscribed_circle_area(&self) -> f64;
    fn circumscribed_circle_area(&self) -> f64;
    fn ratio(&self) -> f64;
}

impl Shape for Polygon {
    fn perimeter(&self) -> f64 {
        self.side * self.side_count
    }
    fn radius(&self) -> f64 {
        self.side / (2.0 * f64::sin(PI / self.side_count))
    }
    fn apothem(&self) -> f64 {
        self.side / (2.0 * f64::tan(PI / self.side_count))
    }
    fn area(&self) -> f64 {
        (self.side * self.side * self.side_count) / (4.0 * f64::tan(PI / self.side_count))
    }
    fn inscribed_circle_area(&self) -> f64 {
        let r_inscribed = self.apothem();
        PI * r_inscribed.powi(2)
    }
    
    fn circumscribed_circle_area(&self) -> f64 {
        let r_circumscribed = self.radius();
        PI * r_circumscribed.powi(2)
    }
    fn ratio(&self) -> f64 {
        self.inscribed_circle_area() / self.area()
    }
}

fn create_shape(side: f64, side_count: f64) -> Box<dyn Shape> {
    Box::new(Polygon{side, side_count})
 }
 
fn main() {
    let mut shapes = Vec::new();
let side_lengths = [1.0, 2.0, 3.0];
let side_counts = [4.0, 8.0, 16.0, 32.0, 64.0, 128.0, 256.0, 512.0, 2048.0, 65536.0];

for &side in &side_lengths {
    for &count in &side_counts {
        shapes.push(create_shape(side, count));
    }
}

    for (i, shape) in shapes.iter().enumerate() {
        println!(
            "Shape {} \n Area: {} \n Perimeter: {} \n Radius: {} \n Apothem: {} \n Inscribed Circle Area: {} \n Circumscribed Circle Area: {} \n Ratio: {}",
            i + 1, shape.area(), shape.perimeter(), shape.radius(), shape.apothem(), shape.inscribed_circle_area(), shape.circumscribed_circle_area(), shape.ratio());
    }
}
