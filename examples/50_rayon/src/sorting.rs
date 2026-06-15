//! 并行排序：`par_sort` / `par_sort_unstable` 是标准库 `sort` 的就地并行版。
//!
//! 和并行迭代器一样，这里也是「换个名字就能并行」：把 `v.sort()` 换成
//! `v.par_sort()`，rayon 会在内部用并行归并把切片排好。排序结果与顺序排序
//! **逐位相同**，因此可以在测试里和 `Vec::sort` 的输出精确对比。

use rayon::prelude::*;

/// 并行排序：消费一个 `Vec`，返回升序排好的新 `Vec`。
///
/// 用的是 `par_sort_unstable`——「不稳定」指相等元素的相对顺序可能改变，
/// 但对 `i64` 这种没有「附带数据」的值类型而言，最终序列与稳定排序完全一致，
/// 而且不稳定版通常更快、内存占用更小。若元素是「带 key 的结构体」且要保持
/// 同 key 的原始先后，则应改用 `par_sort`（稳定版）。
///
/// 无论是否并行，排序的输出是确定的，所以结果等于 `let mut s = v; s.sort();`。
///
/// ```
/// use rt_50_rayon::par_sorted;
/// assert_eq!(par_sorted(vec![3, 1, 2]), vec![1, 2, 3]);
/// ```
pub fn par_sorted(mut v: Vec<i64>) -> Vec<i64> {
    v.par_sort_unstable();
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_a_small_vec() {
        assert_eq!(par_sorted(vec![3, 1, 2, -5, 0]), vec![-5, 0, 1, 2, 3]);
    }

    #[test]
    fn matches_sequential_sort_on_large_input() {
        // 用一个确定的伪随机序列，避免引入额外依赖，同时保证测试可复现。
        let mut state: i64 = 1;
        let data: Vec<i64> = (0..10_000)
            .map(|_| {
                // 线性同余发生器（LCG），纯确定性。
                state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
                state
            })
            .collect();

        let mut expected = data.clone();
        expected.sort();

        assert_eq!(par_sorted(data), expected);
    }

    #[test]
    fn handles_empty_and_single() {
        assert_eq!(par_sorted(vec![]), Vec::<i64>::new());
        assert_eq!(par_sorted(vec![42]), vec![42]);
    }
}
