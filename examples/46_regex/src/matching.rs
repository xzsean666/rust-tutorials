//! 最基础的匹配与查找：判断是否匹配、找出文本里的所有数字。
//!
//! 这里用到 `regex` 的三个核心方法：
//! - `is_match`：只回答「匹不匹配」，返回 `bool`。
//! - `find`：找出**第一个**匹配，返回 `Option<Match>`。
//! - `find_iter`：迭代出**所有**匹配，常配合 `collect` 用。

use regex::Regex;

/// 判断一个字符串是否「看起来像」邮箱地址。
///
/// 注意：真正符合 RFC 的邮箱正则极其复杂，这里只做教学用的近似校验：
/// `用户名@域名.后缀`。模式拆开看：
/// - `[\w.+-]+`：用户名，允许字母数字、点、加号、减号。
/// - `@`：字面量 `@`。
/// - `[\w-]+`：域名主体。
/// - `(?:\.[\w-]+)+`：`(?:...)` 是**非捕获组**，`+` 表示至少一段
///   `.后缀`，于是 `a@b.c`、`a@b.co.uk` 都能通过。
/// - 首尾的 `^` 和 `$` 把匹配「锚定」到整行，避免「字符串里夹着一个邮箱」也算通过。
///
/// 模式一律用原始字符串 `r"..."` 包裹，这样 `\w`、`\.` 里的反斜杠不必再转义。
pub fn is_email(text: &str) -> bool {
    let re = Regex::new(r"^[\w.+-]+@[\w-]+(?:\.[\w-]+)+$").unwrap();
    re.is_match(text)
}

/// 返回文本里出现的**第一个**整数（以字符串形式）。
///
/// `\d+` 匹配「一个或多个数字」。`find` 返回 `Option<Match>`，
/// 用 `map` 把 `Match` 转成 `&str` 再 `to_string`，找不到时返回 `None`。
pub fn first_number(text: &str) -> Option<String> {
    let re = Regex::new(r"\d+").unwrap();
    re.find(text).map(|m| m.as_str().to_string())
}

/// 找出文本里的**所有**整数，按出现顺序收集成 `Vec<i64>`。
///
/// `find_iter` 迭代每一个匹配；`as_str()` 拿到匹配到的子串，再 `parse` 成数字。
/// 这里能稳妥地 `unwrap`，因为 `\d+` 保证匹配到的一定是纯数字串。
pub fn find_numbers(text: &str) -> Vec<i64> {
    let re = Regex::new(r"\d+").unwrap();
    re.find_iter(text)
        .map(|m| m.as_str().parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn email_accepts_valid() {
        assert!(is_email("alice@example.com"));
        assert!(is_email("bob.smith+tag@mail.co.uk"));
    }

    #[test]
    fn email_rejects_invalid() {
        assert!(!is_email("not-an-email"));
        assert!(!is_email("missing@domain")); // 没有「.后缀」
        assert!(!is_email("a @b.com")); // 含空格
    }

    #[test]
    fn first_number_works() {
        assert_eq!(
            first_number("订单 #42，金额 100 元"),
            Some("42".to_string())
        );
        assert_eq!(first_number("没有数字"), None);
    }

    #[test]
    fn find_all_numbers() {
        assert_eq!(find_numbers("买了 3 本书，共 128 元"), vec![3, 128]);
        assert_eq!(find_numbers("纯文本"), Vec::<i64>::new());
    }
}
