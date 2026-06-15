//! 常用集合：`Vec<T>`、`String` 与 `HashMap<K, V>`（外加一点 `HashSet`）。
//!
//! 这三种集合是日常 Rust 的主力，本 crate 按集合类型拆成三个模块：
//! - [`vectors`]：`Vec<T>` 的摘要、过滤排序，以及成绩记录类型 [`Score`]。
//! - [`strings`]：`String` 文本拼接（`push_str` / `format!`）。
//! - [`maps`]：`HashMap<K, V>` 计数与 `HashSet<T>` 去重、集合运算。
//!
//! 运行 `cargo run -p rt_13_collections` 查看带标签的输出。

pub mod maps;
pub mod strings;
pub mod vectors;

pub use maps::{char_frequency, common_count, sorted_unique, word_count};
pub use strings::format_scores;
pub use vectors::{Score, passing_sorted, summarize};
