//! 用 `regex` 这个第三方包演示「正则表达式处理文本」。
//!
//! 本 crate 把功能拆成几个模块，方便对照阅读：
//! - `matching`：最基础的匹配、查找——判断是否匹配、找出全部数字。
//! - `captures`：捕获组、命名组与批量替换——解析日期、给手机号打码。
//!
//! 所有能力都来自 crates.io 上的 `regex` 包（见 `Cargo.toml` 的
//! `[dependencies]`）。正则的核心步骤都是固定三步：用 `Regex::new` 编译模式、
//! 用 `is_match` / `find` / `captures` 之类的方法去匹配文本、再读出结果。
//!
//! 注意：编译一个 `Regex` 是相对昂贵的操作。真实项目里应当「编译一次、反复使用」，
//! 常见做法是用 `std::sync::LazyLock`（或 `once_cell::sync::Lazy`）把它放进静态变量；
//! 本例为了聚焦 API，每个函数内部直接 `Regex::new`，但教程里会专门提醒这一点。

pub mod captures;
pub mod matching;

// 重新导出常用函数，调用方可以直接 `use rt_46_regex::is_email;`。
pub use captures::{Date, mask_phones, parse_date};
pub use matching::{find_numbers, first_number, is_email};
