use std::ops::Neg;

#[derive(Debug, Copy, Clone, PartialEq)]

struct Point<T> {
    x: T,
    y: T,
}
impl<T: Copy + Neg<Output = T>> Point<T> {
    fn clockwise(self) -> Point<T> {
        Point {
            x: self.y,
            y: -self.x,
        }
    }

    fn counterclockwise(self) -> Point<T> {
        Point {
            x: -self.y,
            y: self.x,
        }
    }
}

#[test]
fn test_clockwise_rotation() {
    let point = Point {x: 1, y: 2};
    let expected = Point {x: 2, y: -1};
    assert_eq!(point.clockwise(), expected);
}

#[test]
 fn test_counterclockwise_rotation() {
    let point = Point {x: 3, y: -4};
    let expected = Point {x: 4, y: 3};
    assert_eq!(point.counterclockwise(), expected);
}

#[test]
fn test_f32_clockwise_rotation() {
    let point = Point {x: 3.5, y: 2.0};
    let expected = Point {x: 2.0, y: -3.5};
    assert_eq!(point.clockwise(), expected);
}

#[test]
fn test_f32_counterclockwise_rotation() {
    let point = Point {x: -1.5, y: 4.0};
    let expected = Point {x: -4.0, y: -1.5};
    assert_eq!(point.counterclockwise(), expected);
}


fn main() {
    let p = Point {x: 1.0, y: 2.0};
    let clockwise = p.clockwise();
    let counterclockwise = p.counterclockwise();
    let x = Point {x: 1, y: 2};
    let counterclockwise1 = x.counterclockwise();
    let clockwise1 = x.clockwise();
    
    println!("Original: {:?}", p);
    println!("Clockwise: {:?}", clockwise);
    println!("Counterclockwise: {:?}", counterclockwise);
    println!("Original: {:?}", x);
    println!("Clockwise: {:?}", clockwise1);
    println!("Counterclockwise: {:?}", counterclockwise1);
}