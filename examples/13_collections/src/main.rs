//! 演示 `Vec`、`String`、`HashMap` 与 `HashSet` 的常见用法。
//!
//! 具体函数实现见库 crate `rt_13_collections` 的各模块。

use rt_13_collections::{
    Score, char_frequency, common_count, format_scores, passing_sorted, sorted_unique, summarize,
    word_count,
};

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
