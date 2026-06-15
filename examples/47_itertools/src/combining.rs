//! 组合与排列：把多个迭代器拼到一起、排序、拼接成字符串、求笛卡尔积、按块切分。
//!
//! 这些同样来自扩展 trait `Itertools`，再加上一个宏 `izip!`。

use itertools::{Itertools, izip};

/// 用 `izip!` 同时遍历三个序列，逐项打包成 `(姓名, 年龄, 城市)`。
///
/// 标准库的 `zip` 一次只能拼两个；要拼 3 个以上，`izip!` 宏更直观，
/// 而且不会产生 `((a, b), c)` 这种嵌套元组。任一序列耗尽即停止。
pub fn zip_three<'a>(
    names: &[&'a str],
    ages: &[u32],
    cities: &[&'a str],
) -> Vec<(&'a str, u32, &'a str)> {
    izip!(
        names.iter().copied(),
        ages.iter().copied(),
        cities.iter().copied()
    )
    .collect()
}

/// 排序后收集成 `Vec`。`sorted` 会消费迭代器，内部收集再排序，返回一个新迭代器。
///
/// 当源头不是 `Vec`（比如来自 `filter`/`map` 的链）又想要有序结果时，
/// `.sorted()` 比「先 `collect` 成 `Vec` 再 `sort`」少写一步。
pub fn sorted_ascending(data: &[i32]) -> Vec<i32> {
    data.iter().copied().sorted().collect()
}

/// 按指定的「键」排序：这里按字符串长度从短到长。
pub fn sorted_by_length(words: &[&str]) -> Vec<String> {
    words
        .iter()
        .map(|s| s.to_string())
        .sorted_by_key(|s| s.len())
        .collect()
}

/// 把一串可显示的元素用分隔符拼成一个字符串，类似其他语言的 `join`。
///
/// `join` 内部用 `Display` 格式化每个元素，省去手写 `fold` 或处理「最后一个
/// 不加分隔符」的边界。
pub fn join_with(data: &[i32], sep: &str) -> String {
    data.iter().join(sep)
}

/// 求两个序列的笛卡尔积：对左边每个元素，配上右边的每个元素。
///
/// 结果长度是 `left.len() * right.len()`，常用于「生成所有组合」。
pub fn product_pairs<'a>(left: &[i32], right: &[&'a str]) -> Vec<(i32, &'a str)> {
    left.iter()
        .copied()
        .cartesian_product(right.iter().copied())
        .collect()
}

/// 把序列每 `n` 个切成一块，最后一块可能不足 `n` 个。
///
/// `chunks` 返回惰性分组，所以要在持有引用期间把每块收集出来。
pub fn into_chunks(data: &[i32], n: usize) -> Vec<Vec<i32>> {
    let mut out = Vec::new();
    for chunk in &data.iter().copied().chunks(n) {
        out.push(chunk.collect());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zip_three_stops_at_shortest() {
        let got = zip_three(
            &["小明", "小红", "小刚"],
            &[18, 20],
            &["北京", "上海", "广州"],
        );
        // 年龄只有 2 个，所以只产出 2 项。
        assert_eq!(got, vec![("小明", 18, "北京"), ("小红", 20, "上海")]);
    }

    #[test]
    fn sorted_ascending_orders_values() {
        assert_eq!(sorted_ascending(&[3, 1, 2, 1]), vec![1, 1, 2, 3]);
    }

    #[test]
    fn sorted_by_length_is_stable_by_key() {
        let got = sorted_by_length(&["bbb", "a", "cc", "dddd"]);
        assert_eq!(got, vec!["a", "cc", "bbb", "dddd"]);
    }

    #[test]
    fn join_with_inserts_separator() {
        assert_eq!(join_with(&[1, 2, 3], ", "), "1, 2, 3");
        assert_eq!(join_with(&[], ", "), "");
    }

    #[test]
    fn product_pairs_is_full_cross() {
        let got = product_pairs(&[1, 2], &["x", "y"]);
        assert_eq!(got, vec![(1, "x"), (1, "y"), (2, "x"), (2, "y")]);
    }

    #[test]
    fn into_chunks_keeps_remainder() {
        let got = into_chunks(&[1, 2, 3, 4, 5], 2);
        assert_eq!(got, vec![vec![1, 2], vec![3, 4], vec![5]]);
    }
}
