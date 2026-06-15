//! 分组与去重：用 `itertools` 给迭代器加上「按相邻分组、去重、计数」的能力。
//!
//! 关键点是「扩展 trait（extension trait）」：只要 `use itertools::Itertools;`，
//! 标准库的迭代器就会凭空多出 `.chunk_by()`、`.unique()`、`.counts()` 等方法。
//! 这些方法并不是标准库自带的，而是 `itertools` 通过为所有 `Iterator` 实现
//! `Itertools` trait「补」上去的。

use std::collections::HashMap;

use itertools::Itertools;

/// 把「相邻且相等」的元素压成一组，返回 `(值, 该组长度)` 的列表。
///
/// `chunk_by`（0.14 由旧名 `group_by` 改名而来）只合并**相邻**的相同键，
/// 不会跨越中间不同的元素。例如 `[1, 1, 2, 1]` 会得到三组：`1×2`、`2×1`、`1×1`，
/// 而**不是**把两段 `1` 合并。这点和「先排序再分组」截然不同。
///
/// `chunk_by` 返回的是惰性分组，必须在持有它的引用期间消费，所以这里在
/// `for (key, group) in &iter.chunk_by(...)` 里立即统计长度。
pub fn run_lengths(data: &[i32]) -> Vec<(i32, usize)> {
    let mut out = Vec::new();
    // 注意 `&...chunk_by(...)`：迭代的是分组结果的引用。
    for (key, group) in &data.iter().chunk_by(|&&x| x) {
        out.push((key, group.count()));
    }
    out
}

/// 去掉重复元素，但**保留首次出现的顺序**（不会像 `HashSet` 那样打乱顺序）。
///
/// `unique` 内部用哈希集合记录见过的值，因此元素需要 `Hash + Eq`。
pub fn unique_keep_order(data: &[i32]) -> Vec<i32> {
    data.iter().copied().unique().collect()
}

/// 统计每个元素出现的次数，返回一个 `HashMap<值, 次数>`。
///
/// `counts` 会消费迭代器并一次性建好计数表，比手写
/// `for x in it { *map.entry(x).or_insert(0) += 1; }` 更省事。
pub fn word_counts(words: &[&str]) -> HashMap<String, usize> {
    words.iter().map(|s| s.to_string()).counts()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_lengths_only_merges_adjacent() {
        // 末尾的 1 不会和开头的 1 合并，因为它们不相邻。
        let got = run_lengths(&[1, 1, 2, 2, 2, 1]);
        assert_eq!(got, vec![(1, 2), (2, 3), (1, 1)]);
    }

    #[test]
    fn run_lengths_empty() {
        assert_eq!(run_lengths(&[]), vec![]);
    }

    #[test]
    fn unique_preserves_first_seen_order() {
        let got = unique_keep_order(&[3, 1, 3, 2, 1, 2]);
        assert_eq!(got, vec![3, 1, 2]);
    }

    #[test]
    fn word_counts_tallies_each_key() {
        let got = word_counts(&["a", "b", "a", "c", "a", "b"]);
        assert_eq!(got.get("a"), Some(&3));
        assert_eq!(got.get("b"), Some(&2));
        assert_eq!(got.get("c"), Some(&1));
        assert_eq!(got.get("z"), None);
    }
}
