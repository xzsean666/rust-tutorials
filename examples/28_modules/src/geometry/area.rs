//! area 子模块:面积计算。

/// 矩形面积。
pub fn rectangle(width: f64, height: f64) -> f64 {
    width * height
}

/// 圆面积。
pub fn circle(radius: f64) -> f64 {
    std::f64::consts::PI * radius * radius
}
