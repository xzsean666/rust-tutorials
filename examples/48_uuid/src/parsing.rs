//! 解析与格式化 UUID：字符串与 `Uuid` 之间的相互转换。

use uuid::Uuid;

/// 把字符串解析成 `Uuid`。
///
/// 接受常见的带连字符形式（如 `550e8400-e29b-41d4-a716-446655440000`），
/// 也接受无连字符的 32 位十六进制形式。解析失败返回 [`uuid::Error`]。
///
/// ```
/// let id = rt_48_uuid::parse_id("550e8400-e29b-41d4-a716-446655440000").unwrap();
/// assert_eq!(id.to_string(), "550e8400-e29b-41d4-a716-446655440000");
/// assert!(rt_48_uuid::parse_id("not-a-uuid").is_err());
/// ```
pub fn parse_id(s: &str) -> Result<Uuid, uuid::Error> {
    Uuid::parse_str(s)
}

/// 返回带连字符（hyphenated）的标准文本形式，长度为 36（含 4 个连字符）。
///
/// 这等价于 `id.to_string()`，但写法更直白。
pub fn to_hyphenated(id: &Uuid) -> String {
    id.hyphenated().to_string()
}

/// 返回无连字符（simple）的紧凑文本形式，长度为 32。
///
/// 适合放进 URL、文件名等不便出现连字符的场景。
pub fn to_simple(id: &Uuid) -> String {
    id.simple().to_string()
}

/// 返回全零的 nil UUID（`00000000-0000-0000-0000-000000000000`）。
///
/// 常用作“尚未分配”或“占位”的默认值。
pub fn nil_id() -> Uuid {
    Uuid::nil()
}

#[cfg(test)]
mod tests {
    use super::*;

    // 固定字符串让解析相关测试保持确定性。
    const SAMPLE: &str = "550e8400-e29b-41d4-a716-446655440000";

    #[test]
    fn parse_round_trip() {
        let id = parse_id(SAMPLE).unwrap();
        // 解析后再格式化，应当还原成原字符串。
        assert_eq!(id.to_string(), SAMPLE);
        // 再解析一次也应得到同一个值。
        assert_eq!(parse_id(&id.to_string()), Ok(id));
    }

    #[test]
    fn parse_rejects_garbage() {
        assert!(parse_id("not-a-uuid").is_err());
    }

    #[test]
    fn simple_form_has_no_hyphen() {
        let id = parse_id(SAMPLE).unwrap();
        let simple = to_simple(&id);
        assert_eq!(simple.len(), 32);
        assert!(!simple.contains('-'));
    }

    #[test]
    fn hyphenated_form_matches_to_string() {
        let id = parse_id(SAMPLE).unwrap();
        assert_eq!(to_hyphenated(&id), id.to_string());
        assert_eq!(to_hyphenated(&id).len(), 36);
    }

    #[test]
    fn nil_is_all_zeros() {
        let nil = nil_id();
        assert!(nil.is_nil());
        assert_eq!(nil.as_bytes(), &[0u8; 16]);
    }
}
