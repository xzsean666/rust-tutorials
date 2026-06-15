//! 捕获组、命名组与批量替换：从文本里**提取结构化信息**、再**改写**文本。
//!
//! - `captures`：返回一次匹配里的各个捕获组。
//! - 命名组 `(?<name>...)`：用名字而不是序号读取，可读性更好。
//! - `replace_all`：把所有匹配替换成新内容，常用于脱敏、清洗。

use regex::Regex;

/// 解析出来的日期：年、月、日。
#[derive(Debug, PartialEq, Eq)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

/// 把形如 `2026-06-15` 的字符串解析成 [`Date`]。
///
/// 模式用了**命名捕获组** `(?<year>\d{4})-(?<month>\d{2})-(?<day>\d{2})`：
/// - `\d{4}` 表示「恰好 4 位数字」，`\d{2}` 是 2 位。
/// - `(?<year>...)` 给这一组起名叫 `year`，之后用 `caps.name("year")` 读取，
///   比按序号 `caps[1]` 更不容易记错位置。
///
/// `captures` 返回 `Option<Captures>`：整体匹配失败时是 `None`，于是非法格式
/// 会自然地返回 `None`。`name(...)` 返回 `Option<Match>`，因为命名组都在模式里
/// 且整体已匹配成功，这里可以安心 `unwrap`。
pub fn parse_date(text: &str) -> Option<Date> {
    let re = Regex::new(r"^(?<year>\d{4})-(?<month>\d{2})-(?<day>\d{2})$").unwrap();
    let caps = re.captures(text)?;
    Some(Date {
        year: caps.name("year").unwrap().as_str().parse().unwrap(),
        month: caps.name("month").unwrap().as_str().parse().unwrap(),
        day: caps.name("day").unwrap().as_str().parse().unwrap(),
    })
}

/// 给文本里所有「11 位手机号」打码，中间四位换成 `****`。
///
/// 关键点：
/// - 模式 `(\d{3})\d{4}(\d{4})` 用了两个普通捕获组，分别抓住前 3 位和后 4 位，
///   中间 4 位不进组、直接被替换掉。
/// - `replace_all` 把**每一处**匹配都换掉，返回 `Cow<str>`（可能借用、可能新建），
///   这里用 `into_owned` 转成 `String` 方便返回。
/// - 替换串里的 `${1}` / `${2}` 引用第 1、第 2 个捕获组。写成 `${1}` 而不是 `$1`，
///   是为了避免 `$1****` 被误解析成名为 `1****` 的组名。
pub fn mask_phones(text: &str) -> String {
    let re = Regex::new(r"(\d{3})\d{4}(\d{4})").unwrap();
    re.replace_all(text, "${1}****${2}").into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_date() {
        assert_eq!(
            parse_date("2026-06-15"),
            Some(Date {
                year: 2026,
                month: 6,
                day: 15,
            })
        );
    }

    #[test]
    fn reject_bad_date() {
        assert_eq!(parse_date("2026/06/15"), None); // 分隔符不对
        assert_eq!(parse_date("26-6-15"), None); // 位数不够
        assert_eq!(parse_date("hello"), None);
    }

    #[test]
    fn mask_single_phone() {
        assert_eq!(mask_phones("联系电话 13812345678"), "联系电话 138****5678");
    }

    #[test]
    fn mask_multiple_phones() {
        assert_eq!(
            mask_phones("A:13800000000 B:13911112222"),
            "A:138****0000 B:139****2222"
        );
    }
}
