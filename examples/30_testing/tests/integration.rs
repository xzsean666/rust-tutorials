//! 集成测试:像外部用户一样,只能访问公开 API。

use rt_30_testing::{add, is_even};

#[test]
fn adds_across_crate_boundary() {
    assert_eq!(add(10, 20), 30);
}

#[test]
fn checks_even_across_crate_boundary() {
    assert!(is_even(0));
    assert!(!is_even(-3));
}
