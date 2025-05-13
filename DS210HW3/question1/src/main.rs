use std::f64::consts::PI;

enum Shape {
    Sphere(f64),
    Cube(f64, f64, f64),
    Pyramid(f64, f64, f64), 
}

impl Shape {
    fn volume(&self) -> f64 {
        match self {
            Shape::Sphere(radius) => (4.0 / 3.0) * PI * radius.powi(3),
            Shape::Cube(side1, side2, side3) => side1 * side2 * side3,
            Shape::Pyramid(side1,side2, height) => (side1 * side2 * height) / 3.0,
        }
    }

    fn surface_area(&self) -> f64 {
        match self {
            Shape::Sphere(radius) => 4.0 * PI * radius.powi(2),
            Shape::Cube(side1, side2, side3) => (2.0 * side1 * side2) + (2.0 * side1 * side3) + (2.0 * side2 * side3),
            Shape::Pyramid(side1, side2, height) => {
                (side1 * side2) + 
                side1 * ((side2/2.0).powi(2) + height.powi(2)).sqrt() +
                side2 * ((side1/2.0).powi(2) + height.powi(2)).sqrt()
            }
        }
    }

    fn is_valid(&self) -> bool {
        match self {
            Shape::Sphere(radius) => *radius > 0.0,
            Shape::Cube(side1, side2, side3) => *side1 > 0.0 && *side2 > 0.0 && *side3 > 0.0,
            Shape::Pyramid(side1, side2, height) => *side1 > 0.0 && *side2 > 0.0 && *height > 0.0,
        }
    }
}

fn create_shape(shape_type: &str, dimensions: Vec<f64>) -> Shape {
    let shape = match shape_type {
        "sphere" => Shape::Sphere(dimensions[0]),
        "cube" => Shape::Cube(dimensions[0], dimensions[1], dimensions[2]),
        "pyramid" => Shape::Pyramid(dimensions[0], dimensions[1], dimensions[2]),
        _ => panic!("Invalid shape type"),
    };
    if !shape.is_valid() {
        panic!("Invalid shape parameters");
    }
    shape
}

fn double(shape: &Shape) -> Shape {
    match shape {
        Shape::Cube(side1, side2, side3) => Shape::Cube(side1 * 2.0, side2 * 2.0, side3 * 2.0),
        Shape::Sphere(radius) => Shape::Sphere(radius * 2.0),
        Shape::Pyramid(side1, side2, height) => Shape::Pyramid(side1 * 2.0, side2 * 2.0, height * 2.0),
    }
}

fn main() {
    let shape1 = create_shape("sphere", vec![2.0]);
    let shape2 = create_shape("cube", vec![2.0, 2.0, 2.0]);
    let shape3 = create_shape("pyramid", vec![2.0, 2.0, 3.0]);
    let shape4 = double(&shape1);
    let shape5 = double(&shape2);
    let shape6 = double(&shape3);

    println!("Sphere Volume: {}", shape1.volume());
    println!("Sphere Surface Area: {}", shape1.surface_area());
    println!("Double Sphere Volume: {}", shape4.volume());
    println!("Double Sphere Surface Area: {}", shape4.surface_area());
    println!("Is Sphere Valid: {}", shape1.is_valid());
    
    println!("Cube Volume: {}", shape2.volume());
    println!("Cube Surface Area: {}", shape2.surface_area());
    println!("Double Cube Volume: {}", shape5.volume());
    println!("Double Cube Surface Area: {}", shape5.surface_area());
    println!("Is Cube Valid: {}", shape2.is_valid());

    println!("Pyramid Volume: {}", shape3.volume());
    println!("Pyramid Surface Area: {}", shape3.surface_area());
    println!("Double Pyramid Volume: {}", shape6.volume());
    println!("Double Pyramid Surface Area: {}", shape6.surface_area());
    println!("Is Pyramid Valid: {}", shape3.is_valid());
}