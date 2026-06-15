//! 日期的解析与格式化。
//!
//! - **解析**：用 [`NaiveDate::parse_from_str`] 把字符串按指定格式变成日期。
//! - **格式化**：用 [`NaiveDate::format`] 配合格式串把日期变回字符串。
//!
//! 格式串里的占位符（部分常用）：
//! - `%Y` 四位年、`%m` 两位月、`%d` 两位日。
//! - 普通字符（包括汉字「年月日」）原样输出。

use chrono::NaiveDate;

/// 把 `"2026-06-15"` 这样的字符串解析成 [`NaiveDate`]。
///
/// 解析可能失败（比如格式不符、日期非法），所以返回 `Result`。
///
/// # 示例
///
/// ```
/// use rt_45_chrono::parse_date;
/// let d = parse_date("2026-06-15").unwrap();
/// assert_eq!(d.to_string(), "2026-06-15");
/// ```
pub fn parse_date(s: &str) -> Result<NaiveDate, chrono::ParseError> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
}

/// 把日期格式化成 ISO 风格的 `YYYY-MM-DD` 字符串。
pub fn format_iso(date: NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}

/// 把日期格式化成中文风格的 `YYYY年MM月DD日` 字符串。
pub fn format_cn(date: NaiveDate) -> String {
    date.format("%Y年%m月%d日").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_date() {
        let d = parse_date("2026-06-15").unwrap();
        assert_eq!(d, NaiveDate::from_ymd_opt(2026, 6, 15).unwrap());
    }

    #[test]
    fn parse_invalid_date_is_err() {
        // 2 月没有 30 号，解析应当失败。
        assert!(parse_date("2026-02-30").is_err());
        // 格式不符也失败。
        assert!(parse_date("not-a-date").is_err());
    }

    #[test]
    fn format_round_trip() {
        let d = NaiveDate::from_ymd_opt(2026, 6, 15).unwrap();
        assert_eq!(format_iso(d), "2026-06-15");
        assert_eq!(format_cn(d), "2026年06月15日");
    }
}
