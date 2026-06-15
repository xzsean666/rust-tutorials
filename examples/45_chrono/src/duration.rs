//! 日期运算：加减天数与求差值。
//!
//! 关键类型是 [`chrono::Duration`]，它表示一段「时间间隔」。
//! - 日期 `+ Duration` 得到新日期。
//! - 两个日期相减 `d2 - d1` 得到一个 `Duration`，再用 `num_days()` 取天数。

use chrono::{Duration, NaiveDate};

/// 在 `date` 基础上往后推 `days` 天。
///
/// # 示例
///
/// ```
/// use rt_45_chrono::add_days;
/// use chrono::NaiveDate;
/// let d = NaiveDate::from_ymd_opt(2026, 6, 15).unwrap();
/// assert_eq!(add_days(d, 10), NaiveDate::from_ymd_opt(2026, 6, 25).unwrap());
/// ```
pub fn add_days(date: NaiveDate, days: i64) -> NaiveDate {
    date + Duration::days(days)
}

/// 在 `date` 基础上往前推 `days` 天。
pub fn sub_days(date: NaiveDate, days: i64) -> NaiveDate {
    date - Duration::days(days)
}

/// 求 `from` 到 `to` 之间相差的天数。
///
/// 若 `to` 在 `from` 之后结果为正，反之为负。
///
/// # 示例
///
/// ```
/// use rt_45_chrono::days_between;
/// use chrono::NaiveDate;
/// let a = NaiveDate::from_ymd_opt(2026, 6, 15).unwrap();
/// let b = NaiveDate::from_ymd_opt(2026, 6, 25).unwrap();
/// assert_eq!(days_between(a, b), 10);
/// ```
pub fn days_between(from: NaiveDate, to: NaiveDate) -> i64 {
    (to - from).num_days()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).unwrap()
    }

    #[test]
    fn add_and_sub_cross_month() {
        // 跨月加法：6 月只有 30 天，6/25 + 10 天 = 7/5。
        assert_eq!(add_days(d(2026, 6, 25), 10), d(2026, 7, 5));
        // 减法回到原点。
        assert_eq!(sub_days(d(2026, 7, 5), 10), d(2026, 6, 25));
    }

    #[test]
    fn days_between_sign() {
        assert_eq!(days_between(d(2026, 6, 15), d(2026, 6, 25)), 10);
        // 反向为负。
        assert_eq!(days_between(d(2026, 6, 25), d(2026, 6, 15)), -10);
        // 相同日期为 0。
        assert_eq!(days_between(d(2026, 6, 15), d(2026, 6, 15)), 0);
    }

    #[test]
    fn days_between_cross_year() {
        // 2024 是闰年，2024-2-29 存在；这里跨年算整年。
        assert_eq!(days_between(d(2025, 1, 1), d(2026, 1, 1)), 365);
    }
}
