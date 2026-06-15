//! 常用集合：`Vec<T>`、`String` 与 `HashMap<K, V>`（外加一点 `HashSet`）。
//!
//! 这三种集合是日常 Rust 的主力：
//! - `Vec<T>`：可增长的数组，支持下标、迭代、过滤、排序。
//! - `String`：可增长的 UTF-8 文本，可用 `push_str` / `format!` 拼接。
//! - `HashMap<K, V>`：键值映射，`entry` API 让“查找或插入”非常优雅。
//!
//! 运行 `cargo run -p rt_13_collections` 查看带标签的输出。

use std::collections::{HashMap, HashSet};

/// 一次成绩记录：学生名字与分数。
#[derive(Debug, Clone)]
struct Score {
    name: String,
    points: u32,
}

/// 对一组分数做摘要：数量、总和、最大值与平均值。
///
/// 演示 `Vec` 的迭代器：`iter` / `sum` / `max` / 长度。
/// 空切片时平均值返回 `0.0`，避免除零。
fn summarize(scores: &[u32]) -> (usize, u32, u32, f64) {
    let count = scores.len();
    let total: u32 = scores.iter().sum();
    let max = scores.iter().copied().max().unwrap_or(0);
    let avg = if count == 0 {
        0.0
    } else {
        f64::from(total) / count as f64
    };
    (count, total, max, avg)
}

/// 保留及格（>= 60）的分数，并从高到低排序，返回新的 `Vec`。
///
/// 演示 `retain`（原地过滤）与 `sort_by`（自定义排序）。
fn passing_sorted(scores: &[u32]) -> Vec<u32> {
    let mut result = scores.to_vec();
    result.retain(|&s| s >= 60);
    result.sort_by(|a, b| b.cmp(a)); // 降序
    result
}

/// 把一组成绩拼成一行可读文本，例如 `"Alice: 90, Bob: 55"`。
///
/// 演示 `String` 的构建：`push_str` 与 `format!`。
fn format_scores(scores: &[Score]) -> String {
    let mut line = String::new();
    for (i, score) in scores.iter().enumerate() {
        if i > 0 {
            line.push_str(", ");
        }
        line.push_str(&format!("{}: {}", score.name, score.points));
    }
    line
}

/// 统计每个单词出现的次数。
///
/// `entry(key).or_insert(0)` 返回值的可变引用：键不存在时插入 `0`，
/// 存在时直接拿到，于是 `+= 1` 就完成了计数。
fn word_count(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word.to_string()).or_insert(0) += 1;
    }
    counts
}

/// 统计每个字符出现的次数（忽略空白字符）。
///
/// 演示 `String`/`&str` 的 `chars` 迭代与 `HashMap<char, usize>`。
fn char_frequency(text: &str) -> HashMap<char, usize> {
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
fn sorted_unique(values: &[i32]) -> Vec<i32> {
    let mut result = values.to_vec();
    result.sort_unstable();
    result.dedup();
    result
}

/// 用 `HashSet` 求两个切片的去重交集元素个数。
///
/// 演示 `HashSet` 的快速成员判断与集合运算。
fn common_count(a: &[i32], b: &[i32]) -> usize {
    let set_a: HashSet<i32> = a.iter().copied().collect();
    let set_b: HashSet<i32> = b.iter().copied().collect();
    set_a.intersection(&set_b).count()
}

fn main() {
    // ---------- Vec：摘要与过滤 ----------
    let raw = [90, 55, 72, 60, 48, 88];
    println!("== Vec ==");
    println!("原始分数: {raw:?}");

    let (count, total, max, avg) = summarize(&raw);
    println!("数量={count}, 总分={total}, 最高={max}, 平均={avg:.1}");
    println!("及格并降序: {:?}", passing_sorted(&raw));

    // push / pop / 下标访问 / 安全访问
    let mut stack = vec![10, 20, 30];
    stack.push(40);
    println!("push 后: {stack:?}");
    println!("stack[0] = {}", stack[0]); // 下标越界会 panic
    println!("stack.get(99) = {:?}", stack.get(99)); // 越界返回 None
    println!("pop 出: {:?}", stack.pop());

    // ---------- String：构建文本 ----------
    println!("\n== String ==");
    let scores = vec![
        Score {
            name: "Alice".to_string(),
            points: 90,
        },
        Score {
            name: "Bob".to_string(),
            points: 55,
        },
    ];
    println!("成绩单: {}", format_scores(&scores));

    // ---------- HashMap：计数 ----------
    println!("\n== HashMap ==");
    let counts = word_count("the cat the dog the bird");
    println!("'the' 出现 {} 次", counts.get("the").copied().unwrap_or(0));
    // 排序后输出，保证演示结果稳定（HashMap 迭代顺序不固定）
    let mut pairs: Vec<_> = counts.iter().collect();
    pairs.sort();
    println!("全部词频: {pairs:?}");

    let freq = char_frequency("hello");
    println!("'l' 出现 {} 次", freq.get(&'l').copied().unwrap_or(0));

    // ---------- HashSet：去重与交集 ----------
    println!("\n== HashSet ==");
    let unique = sorted_unique(&[3, 1, 2, 3, 1]);
    println!("排序去重: {unique:?}");
    println!(
        "[1,2,3,4] 与 [3,4,5] 的公共元素个数: {}",
        common_count(&[1, 2, 3, 4], &[3, 4, 5])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summarizes_scores() {
        let (count, total, max, avg) = summarize(&[10, 20, 30]);
        assert_eq!(count, 3);
        assert_eq!(total, 60);
        assert_eq!(max, 30);
        assert!((avg - 20.0).abs() < f64::EPSILON);
    }

    #[test]
    fn summarize_handles_empty() {
        let (count, total, max, avg) = summarize(&[]);
        assert_eq!((count, total, max), (0, 0, 0));
        assert_eq!(avg, 0.0);
    }

    #[test]
    fn keeps_passing_and_sorts_desc() {
        assert_eq!(passing_sorted(&[90, 55, 60, 48]), vec![90, 60]);
        assert_eq!(passing_sorted(&[10, 20]), Vec::<u32>::new());
    }

    #[test]
    fn formats_scores_line() {
        let scores = vec![
            Score {
                name: "Alice".to_string(),
                points: 90,
            },
            Score {
                name: "Bob".to_string(),
                points: 55,
            },
        ];
        assert_eq!(format_scores(&scores), "Alice: 90, Bob: 55");
        assert_eq!(format_scores(&[]), "");
    }

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
