//! 迭代器与适配器：用声明式的链式调用替代手写循环。
//!
//! 核心心智模型：
//! - **适配器（adapter）** 如 `map` / `filter` / `zip`，输入一个迭代器、返回一个新的迭代器，
//!   它们都是**惰性的**：在被消费之前不做任何实际工作。
//! - **消费者（consumer）** 如 `collect` / `sum` / `fold` / `find`，真正驱动迭代并产出结果。
//! - 三种取值方式：`iter()` 产出 `&T`，`iter_mut()` 产出 `&mut T`，`into_iter()` 产出 `T`（消费集合）。

use std::collections::HashMap;

/// `map` + `filter` + `sum`：偶数平方之和。
///
/// 链式适配器组合：先筛掉奇数，再平方，最后求和。
/// 全程惰性——直到 `sum()` 才真正遍历一次。
fn sum_of_even_squares(values: &[i32]) -> i32 {
    values.iter().filter(|&&v| v % 2 == 0).map(|&v| v * v).sum()
}

/// `filter_map`：把能解析成 `i32` 的字符串收集出来，无法解析的直接丢弃。
///
/// `filter_map` = `filter` + `map` 的融合：闭包返回 `Option`，`Some` 保留、`None` 丢弃。
fn parse_numbers(tokens: &[&str]) -> Vec<i32> {
    tokens
        .iter()
        .filter_map(|t| t.parse::<i32>().ok())
        .collect()
}

/// `enumerate` + `zip`：把序号、名字、分数三者对齐成一行行文本。
///
/// `enumerate` 给出 `(下标, 元素)`；`zip` 把两个迭代器配对，长度取较短者。
fn rank_lines(names: &[&str], scores: &[i32]) -> Vec<String> {
    names
        .iter()
        .zip(scores.iter())
        .enumerate()
        .map(|(i, (name, score))| format!("#{} {name}: {score}", i + 1))
        .collect()
}

/// `flat_map`：把嵌套结构摊平成一维。
///
/// 这里把若干句子拆成单词，再汇成一个扁平的 `Vec`。
fn all_words<'a>(sentences: &[&'a str]) -> Vec<&'a str> {
    sentences
        .iter()
        .flat_map(|s| s.split_whitespace())
        .collect()
}

/// `skip` + `take` + `chain`：跳过表头、取若干行，再拼上一段尾注。
///
/// `skip(n)` 跳过前 n 个，`take(n)` 只取前 n 个，`chain` 把两个迭代器首尾相接。
fn paginate<'a>(rows: &[&'a str], footer: &[&'a str]) -> Vec<&'a str> {
    rows.iter()
        .copied()
        .skip(1)
        .take(2)
        .chain(footer.iter().copied())
        .collect()
}

/// `collect` 进 `HashMap`：用单词建立“单词 -> 长度”的映射。
///
/// 迭代器产出 `(K, V)` 元组时，`collect()` 可直接收成 `HashMap`。
fn word_lengths(words: &[&str]) -> HashMap<String, usize> {
    words.iter().map(|&w| (w.to_string(), w.len())).collect()
}

/// `collect` 进 `String`：把字符迭代器拼成字符串（顺带大写化）。
fn shout(text: &str) -> String {
    text.chars().map(|c| c.to_ascii_uppercase()).collect()
}

/// `fold`：最通用的消费者，一次遍历同时算出最小值与最大值。
fn min_max(values: &[i32]) -> Option<(i32, i32)> {
    values.iter().fold(None, |acc, &v| match acc {
        None => Some((v, v)),
        Some((min, max)) => Some((min.min(v), max.max(v))),
    })
}

/// 一组短路消费者：`any` / `all` / `find` / `count`。
///
/// 它们在拿到结论后会**提前停止**，不必遍历完整个序列。
fn analyze(values: &[i32]) -> (bool, bool, Option<i32>, usize) {
    let has_negative = values.iter().any(|&v| v < 0);
    let all_small = values.iter().all(|&v| v.abs() < 100);
    let first_even = values.iter().copied().find(|&v| v % 2 == 0);
    let positive_count = values.iter().filter(|&&v| v > 0).count();
    (has_negative, all_small, first_even, positive_count)
}

/// `iter_mut`：原地修改集合里的每个元素（这里把每个值翻倍）。
fn double_in_place(values: &mut [i32]) {
    values.iter_mut().for_each(|v| *v *= 2);
}

fn main() {
    let numbers = [1, 2, 3, 4, 5, 6];
    println!("== 适配器组合 ==");
    println!("偶数平方之和 = {}", sum_of_even_squares(&numbers));
    println!("解析数字     = {:?}", parse_numbers(&["1", "x", "3", "九"]));
    println!(
        "摊平单词     = {:?}",
        all_words(&["hello world", "foo bar"])
    );

    println!("\n== 对齐与分页 ==");
    for line in rank_lines(&["Amy", "Bob", "Cid"], &[90, 85, 88]) {
        println!("{line}");
    }
    println!(
        "分页结果 = {:?}",
        paginate(&["header", "r1", "r2", "r3"], &["--end--"])
    );

    println!("\n== 收集到不同容器 ==");
    println!("单词长度映射 = {:?}", word_lengths(&["hi", "rust"]));
    println!("大写字符串   = {}", shout("hello"));

    println!("\n== 消费者 ==");
    println!("最小/最大 = {:?}", min_max(&numbers));
    let (neg, small, even, pos) = analyze(&numbers);
    println!("有负数={neg} 全小于100={small} 首个偶数={even:?} 正数个数={pos}");

    let mut owned = numbers.to_vec();
    double_in_place(&mut owned);
    println!("原地翻倍 = {owned:?}");
    // into_iter() 消费 owned，产出 T（这里是 i32）而非 &T。
    let total: i32 = owned.into_iter().sum();
    println!("翻倍后求和 = {total}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_even_squares() {
        // 偶数 2,4,6 -> 4+16+36 = 56
        assert_eq!(sum_of_even_squares(&[1, 2, 3, 4, 5, 6]), 56);
        assert_eq!(sum_of_even_squares(&[1, 3, 5]), 0);
    }

    #[test]
    fn parses_only_valid_numbers() {
        assert_eq!(parse_numbers(&["1", "x", "3", "九"]), vec![1, 3]);
        assert_eq!(parse_numbers(&[]), Vec::<i32>::new());
    }

    #[test]
    fn ranks_and_zips() {
        let lines = rank_lines(&["Amy", "Bob"], &[90, 85]);
        assert_eq!(lines, vec!["#1 Amy: 90", "#2 Bob: 85"]);
        // zip 以较短者为准：names 比 scores 多一个，多出的被忽略。
        assert_eq!(rank_lines(&["A", "B"], &[1]).len(), 1);
    }

    #[test]
    fn flattens_words() {
        assert_eq!(all_words(&["a b", "c"]), vec!["a", "b", "c"]);
    }

    #[test]
    fn paginates_with_chain() {
        let out = paginate(&["header", "r1", "r2", "r3"], &["end"]);
        assert_eq!(out, vec!["r1", "r2", "end"]);
    }

    #[test]
    fn collects_into_map_and_string() {
        let map = word_lengths(&["hi", "rust"]);
        assert_eq!(map.get("hi"), Some(&2));
        assert_eq!(map.get("rust"), Some(&4));
        assert_eq!(shout("ab"), "AB");
    }

    #[test]
    fn folds_min_max() {
        assert_eq!(min_max(&[3, 1, 4, 1, 5]), Some((1, 5)));
        assert_eq!(min_max(&[]), None);
    }

    #[test]
    fn short_circuit_consumers() {
        let (neg, small, even, pos) = analyze(&[1, 2, 3, -4]);
        assert!(neg);
        assert!(small);
        assert_eq!(even, Some(2));
        assert_eq!(pos, 3);
    }

    #[test]
    fn mutates_in_place() {
        let mut v = [1, 2, 3];
        double_in_place(&mut v);
        assert_eq!(v, [2, 4, 6]);
    }
}
