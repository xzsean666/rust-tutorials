//! trait 对象：用 dyn 在运行时存放不同类型。

trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &str;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    fn name(&self) -> &str {
        "circle"
    }
}

struct Square {
    side: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
    fn name(&self) -> &str {
        "square"
    }
}

/// 一个 Vec 里放多种实现了 Shape 的类型。
fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|shape| shape.area()).sum()
}

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 1.0 }),
        Box::new(Square { side: 2.0 }),
    ];
    for shape in &shapes {
        println!("{} => {:.2}", shape.name(), shape.area());
    }
    println!("total = {:.2}", total_area(&shapes));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_mixed_shapes() {
        let shapes: Vec<Box<dyn Shape>> = vec![
            Box::new(Square { side: 2.0 }),
            Box::new(Square { side: 3.0 }),
        ];
        assert_eq!(total_area(&shapes), 13.0);
    }

    #[test]
    fn reports_name() {
        let circle = Circle { radius: 1.0 };
        assert_eq!(circle.name(), "circle");
    }
}
