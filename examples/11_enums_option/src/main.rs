//! 带数据的枚举与 Option。

#[derive(Debug, PartialEq)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
        }
    }
}

/// 用 Option 表达“可能没有结果”。
fn first_even(values: &[i32]) -> Option<i32> {
    values.iter().copied().find(|value| value % 2 == 0)
}

fn main() {
    let shapes = [
        Shape::Circle { radius: 1.0 },
        Shape::Rectangle {
            width: 2.0,
            height: 3.0,
        },
    ];
    for shape in &shapes {
        println!("{shape:?} => area {:.2}", shape.area());
    }

    match first_even(&[1, 3, 4, 7]) {
        Some(value) => println!("first even = {value}"),
        None => println!("no even number"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rectangle_area() {
        let shape = Shape::Rectangle {
            width: 2.0,
            height: 3.0,
        };
        assert_eq!(shape.area(), 6.0);
    }

    #[test]
    fn circle_area_is_positive() {
        let shape = Shape::Circle { radius: 1.0 };
        assert!((shape.area() - std::f64::consts::PI).abs() < 1e-9);
    }

    #[test]
    fn finds_first_even() {
        assert_eq!(first_even(&[1, 3, 4, 7]), Some(4));
        assert_eq!(first_even(&[1, 3, 5]), None);
    }
}
