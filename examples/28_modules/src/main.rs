//! 演示模块系统:mod 声明、pub 可见性、use 引入。

mod geometry;

use geometry::area;

fn main() {
    println!("{}", geometry::describe());
    println!("rectangle = {}", area::rectangle(2.0, 3.0));
    println!("circle = {:.2}", area::circle(1.0));
}

#[cfg(test)]
mod tests {
    use crate::geometry::area;

    #[test]
    fn rectangle_area() {
        assert_eq!(area::rectangle(2.0, 3.0), 6.0);
    }

    #[test]
    fn circle_area() {
        assert!((area::circle(1.0) - std::f64::consts::PI).abs() < 1e-9);
    }
}
