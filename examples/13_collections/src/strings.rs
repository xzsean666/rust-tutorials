//! `String` 相关：把结构化数据拼接成可读文本。
//!
//! `String` 是可增长的 UTF-8 文本，可用 `push_str` / `format!` 拼接。

use crate::vectors::Score;

/// 把一组成绩拼成一行可读文本，例如 `"Alice: 90, Bob: 55"`。
///
/// 演示 `String` 的构建：`push_str` 与 `format!`。
pub fn format_scores(scores: &[Score]) -> String {
    let mut line = String::new();
    for (i, score) in scores.iter().enumerate() {
        if i > 0 {
            line.push_str(", ");
        }
        line.push_str(&format!("{}: {}", score.name, score.points));
    }
    line
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
