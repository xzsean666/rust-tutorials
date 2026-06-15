//! 图形相关：`Shape` trait、`Circle`/`Rectangle` 类型、`Display` 实现，
//! 以及操作 `Shape` 的函数。
//!
//! 本模块演示：
//! - 用 trait bound 写通用算法；
//! - 为自定义类型实现标准库 trait（`std::fmt::Display`）。

use std::fmt;

/// 一个带“面积”行为的 trait，演示用 trait bound 写通用算法。
pub trait Shape {
    /// 返回图形面积。
    fn area(&self) -> f64;

    /// 默认方法：名称，默认未知。
    fn name(&self) -> &str {
        "图形"
    }
}

/// 圆形。
pub struct Circle {
    pub radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn name(&self) -> &str {
        "圆形"
    }
}

/// 矩形。
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn name(&self) -> &str {
        "矩形"
    }
}

/// 为自定义类型实现标准库 trait `Display`，让它能被 `{}` 直接打印。
impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}×{} 的矩形", self.width, self.height)
    }
}

/// 泛型函数 + trait bound：对任意 `Shape` 给出一句描述。
pub fn describe_shape<S: Shape>(shape: &S) -> String {
    format!("{} 的面积是 {:.2}", shape.name(), shape.area())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shape_area_and_name() {
        let rect = Rectangle {
            width: 3.0,
            height: 4.0,
        };
        assert_eq!(rect.name(), "矩形");
        assert!((rect.area() - 12.0).abs() < 1e-9);
    }

    #[test]
    fn display_for_rectangle() {
        let rect = Rectangle {
            width: 3.0,
            height: 4.0,
        };
        assert_eq!(format!("{rect}"), "3×4 的矩形");
    }
}
