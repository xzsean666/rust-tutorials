//! 随机密码：从一个字符集里反复随机挑字符，拼成指定长度的密码。

use rand::seq::IndexedRandom;

/// 生成密码时可选用的字符集合（大小写字母 + 数字）。
pub const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                             abcdefghijklmnopqrstuvwxyz\
                             0123456789";

/// 生成一个长度为 `len` 的随机密码。
///
/// 核心 API 是切片的 `.choose(&mut rng)`：它来自 `rand::seq::IndexedRandom`
/// 这个 trait，所以必须先 `use` 它才能调用。`choose` 返回 `Option<&T>`
/// （切片可能为空），这里 `CHARSET` 非空，直接 `expect` 即可。
pub fn generate_password(len: usize) -> String {
    let mut rng = rand::rng();
    (0..len)
        .map(|_| {
            let &byte = CHARSET.choose(&mut rng).expect("字符集不应为空");
            byte as char
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_has_expected_length() {
        for len in [0, 1, 8, 32] {
            assert_eq!(generate_password(len).chars().count(), len);
        }
    }

    #[test]
    fn password_uses_only_charset() {
        let pw = generate_password(200);
        assert!(
            pw.bytes().all(|b| CHARSET.contains(&b)),
            "出现了字符集之外的字符: {pw}"
        );
    }
}
