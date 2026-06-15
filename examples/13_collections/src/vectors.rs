//! `Vec<T>` 相关：摘要、过滤排序，以及成绩记录类型 `Score`。
//!
//! `Vec<T>` 是可增长的数组，支持下标、迭代、过滤与排序。

/// 一次成绩记录：学生名字与分数。
#[derive(Debug, Clone)]
pub struct Score {
    pub name: String,
    pub points: u32,
}

/// 对一组分数做摘要：数量、总和、最大值与平均值。
///
/// 演示 `Vec` 的迭代器：`iter` / `sum` / `max` / 长度。
/// 空切片时平均值返回 `0.0`，避免除零。
pub fn summarize(scores: &[u32]) -> (usize, u32, u32, f64) {
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
pub fn passing_sorted(scores: &[u32]) -> Vec<u32> {
    let mut result = scores.to_vec();
    result.retain(|&s| s >= 60);
    result.sort_by(|a, b| b.cmp(a)); // 降序
    result
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
}
