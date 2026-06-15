//! Vec、HashMap 与 String 常用操作。

use std::collections::HashMap;

/// 统计每个单词出现的次数。
fn word_count(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word.to_string()).or_insert(0) += 1;
    }
    counts
}

/// 排序去重，返回新的 Vec。
fn sorted_unique(values: &[i32]) -> Vec<i32> {
    let mut result = values.to_vec();
    result.sort_unstable();
    result.dedup();
    result
}

fn main() {
    let counts = word_count("the cat the dog the bird");
    println!("'the' 出现 {} 次", counts.get("the").copied().unwrap_or(0));

    let unique = sorted_unique(&[3, 1, 2, 3, 1]);
    println!("sorted unique = {unique:?}");
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
    fn sorts_and_dedups() {
        assert_eq!(sorted_unique(&[3, 1, 2, 3, 1]), vec![1, 2, 3]);
        assert_eq!(sorted_unique(&[]), Vec::<i32>::new());
    }
}
