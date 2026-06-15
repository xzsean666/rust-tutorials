//! 用 `itertools` 这个第三方包给标准迭代器「加料」。
//!
//! 标准库的 `Iterator` 已经很强（`map`/`filter`/`fold`……），但有些常见操作
//! 它没有直接提供，比如「按相邻分组」「去重保序」「计数」「拼接成字符串」
//! 「笛卡尔积」。`itertools` 用**扩展 trait** 的方式把这些方法补上：
//! 只要 `use itertools::Itertools;`，所有迭代器就立刻多出这些方法，无需改动
//! 原有类型。
//!
//! 本 crate 按用途分成两个模块：
//! - `grouping`：分组与去重——`chunk_by`、`unique`、`counts`。
//! - `combining`：组合与排列——`izip!`、`sorted`、`join`、`cartesian_product`、`chunks`。

pub mod combining;
pub mod grouping;

// 重新导出常用函数，调用方可以直接 `use rt_47_itertools::run_lengths;`。
pub use combining::{
    into_chunks, join_with, product_pairs, sorted_ascending, sorted_by_length, zip_three,
};
pub use grouping::{run_lengths, unique_keep_order, word_counts};
