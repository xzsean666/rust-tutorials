//! 用 `chrono` crate 处理日期与时间。
//!
//! 标准库的 `std::time` 只提供「时间点 / 时间间隔」这类底层能力，
//! 既不能解析 `"2026-06-15"` 这样的日期字符串，也不会按「年月日」做日历运算。
//! `chrono` 在其之上提供了完整的日历日期、时区与格式化支持。
//!
//! 本 crate 把功能拆成两个模块逐一演示：
//! - `parsing`：解析与格式化（把字符串变成日期，再把日期格式化成字符串）。
//! - `duration`：日期运算（加减天数、求两个日期之间相差的天数）。
//!
//! 为了让测试结果**确定**，示例统一使用不带时区的 [`chrono::NaiveDate`]，
//! 而不是依赖「当前时间」`Utc::now()` / `Local::now()`。

pub mod duration;
pub mod parsing;

// 重新导出常用函数，调用方可以直接 `use rt_45_chrono::parse_date;`。
pub use duration::{add_days, days_between, sub_days};
pub use parsing::{format_cn, format_iso, parse_date};
