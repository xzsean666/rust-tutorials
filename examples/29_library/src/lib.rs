//! 一个最小库 crate:温度单位转换。
//!
//! 库 crate 通过 `lib.rs` 对外暴露 API,二进制或其他 crate 都能复用。

/// 摄氏度转华氏度。
pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

/// 华氏度转摄氏度。
pub fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_to_fahrenheit() {
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
        assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
    }

    #[test]
    fn round_trips() {
        let back = fahrenheit_to_celsius(celsius_to_fahrenheit(37.0));
        assert!((back - 37.0).abs() < 1e-9);
    }
}
