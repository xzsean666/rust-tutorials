//! `HashMap<K, V>` 与 `HashSet<T>` 相关：计数、去重与集合运算。
//!
//! - `HashMap<K, V>`：键值映射，`entry` API 让“查找或插入”非常优雅。
//! - `HashSet<T>`：快速成员判断与集合运算（交集、并集等）。

use std::collections::{HashMap, HashSet};

/// 统计每个单词出现的次数。
///
/// `entry(key).or_insert(0)` 返回值的可变引用：键不存在时插入 `0`，
/// 存在时直接拿到，于是 `+= 1` 就完成了计数。
pub fn word_count(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word.to_string()).or_insert(0) += 1;
    }
    counts
}

/// 统计每个字符出现的次数（忽略空白字符）。
///
/// 演示 `String`/`&str` 的 `chars` 迭代与 `HashMap<char, usize>`。
pub fn char_frequency(text: &str) -> HashMap<char, usize> {
    let mut freq = HashMap::new();
    for ch in text.chars().filter(|c| !c.is_whitespace()) {
        *freq.entry(ch).or_insert(0) += 1;
    }
    freq
}

/// 排序去重，返回新的 `Vec`。
///
/// `dedup` 只能去掉**相邻**重复，所以必须先 `sort` 才能彻底去重；
/// 这里也演示了如何用 `HashSet` 达到同样效果（但顺序不保证）。
pub fn sorted_unique(values: &[i32]) -> Vec<i32> {
    let mut result = values.to_vec();
    result.sort_unstable();
    result.dedup();
    result
}

/// 用 `HashSet` 求两个切片的去重交集元素个数。
///
/// 演示 `HashSet` 的快速成员判断与集合运算。
pub fn common_count(a: &[i32], b: &[i32]) -> usize {
    let set_a: HashSet<i32> = a.iter().copied().collect();
    let set_b: HashSet<i32> = b.iter().copied().collect();
    set_a.intersection(&set_b).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_words() {
        let counts = word_count("a b a c a");
        assert_eq!(counts.get("a"), Some(&3));
        assert_eq!(counts.get("b"), Some(&1));
        assert_eq!(counts.get("z"), None);
    }

    #[test]
    fn counts_chars() {
        let freq = char_frequency("hello world");
        assert_eq!(freq.get(&'l'), Some(&3));
        assert_eq!(freq.get(&'o'), Some(&2));
        assert_eq!(freq.get(&' '), None); // 空白被忽略
    }

    #[test]
    fn sorts_and_dedups() {
        assert_eq!(sorted_unique(&[3, 1, 2, 3, 1]), vec![1, 2, 3]);
        assert_eq!(sorted_unique(&[]), Vec::<i32>::new());
    }

    #[test]
    fn counts_common_elements() {
        assert_eq!(common_count(&[1, 2, 3, 4], &[3, 4, 5]), 2);
        assert_eq!(common_count(&[1, 2], &[3, 4]), 0);
    }
}
