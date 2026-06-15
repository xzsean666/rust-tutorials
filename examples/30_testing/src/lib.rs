//! 演示单元测试与集成测试。

/// 两数相加。
pub fn add(left: i64, right: i64) -> i64 {
    left + right
}

/// 判断是否为偶数。
pub fn is_even(value: i64) -> bool {
    value % 2 == 0
}

// 单元测试:和被测代码放在同一文件,可访问私有项。
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn detects_even() {
        assert!(is_even(4));
        assert!(!is_even(5));
    }

    #[test]
    #[should_panic(expected = "boom")]
    fn can_assert_panic() {
        panic!("boom");
    }
}
