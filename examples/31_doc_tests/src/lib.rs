//! 演示文档注释与文档测试。
//!
//! `cargo test` 会编译并运行文档里的示例代码,保证文档不会过时。

/// 返回两个数中较大的一个。
///
/// # 示例
///
/// ```
/// use rt_31_doc_tests::max;
/// assert_eq!(max(3, 7), 7);
/// assert_eq!(max(-1, -5), -1);
/// ```
pub fn max(left: i64, right: i64) -> i64 {
    if left >= right { left } else { right }
}

/// 反转字符串。
///
/// # 示例
///
/// ```
/// use rt_31_doc_tests::reverse;
/// assert_eq!(reverse("abc"), "cba");
/// ```
pub fn reverse(text: &str) -> String {
    text.chars().rev().collect()
}
