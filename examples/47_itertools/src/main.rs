//! 一个小演示：调用本 crate 暴露的 itertools 扩展功能，并打印带标签的输出。
//!
//! `main` 只管调用 `rt_47_itertools` 里封装好的函数——这些函数内部用到了
//! 第三方包 `itertools`，但调用方完全感知不到。

use rt_47_itertools::{
    into_chunks, join_with, product_pairs, run_lengths, sorted_by_length, unique_keep_order,
    word_counts, zip_three,
};

fn main() {
    println!("=== itertools：增强迭代器演示 ===\n");

    println!("-- 分组与去重 --");
    println!("相邻分组(游程): {:?}", run_lengths(&[1, 1, 2, 2, 2, 1]));
    println!(
        "去重保序:       {:?}",
        unique_keep_order(&[3, 1, 3, 2, 1, 2])
    );
    let counts = word_counts(&["a", "b", "a", "c", "a", "b"]);
    println!("词频(a 出现):   {} 次", counts["a"]);

    println!();
    println!("-- 组合与排列 --");
    let people = zip_three(&["小明", "小红"], &[18, 20], &["北京", "上海"]);
    println!("izip! 三路压缩: {people:?}");
    println!(
        "按长度排序:     {:?}",
        sorted_by_length(&["bbb", "a", "cc", "dddd"])
    );
    println!("join 拼接:      {}", join_with(&[1, 2, 3], " - "));
    println!("笛卡尔积:       {:?}", product_pairs(&[1, 2], &["x", "y"]));
    println!("每 2 个一块:    {:?}", into_chunks(&[1, 2, 3, 4, 5], 2));
}
