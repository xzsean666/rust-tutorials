//! 计算 SHA-2 系列摘要，并编码成十六进制字符串。
//!
//! `sha2` crate 把每种哈希算法做成一个类型（`Sha256`、`Sha512` ……），
//! 它们都实现了同一个 [`Digest`](sha2::Digest) trait。因此“算哈希”的
//! 流程对所有算法完全一致，换算法只需换类型名。

use sha2::{Digest, Sha256, Sha512};

/// 计算一段字节数据的 SHA-256 摘要，返回小写十六进制字符串。
///
/// 计算分三步，所有 RustCrypto 的哈希都是这个套路：
/// 1. `Sha256::new()` 创建一个 hasher；
/// 2. `update(data)` 喂入数据（可多次调用，等价于把数据拼接起来）；
/// 3. `finalize()` 得到固定长度的摘要（SHA-256 是 32 字节）。
///
/// 最后用 `hex::encode` 把这 32 个字节转成 64 个十六进制字符。
///
/// ```
/// // SHA-256("abc") 是一个广为人知的测试向量。
/// let h = rt_49_sha2::sha256_hex(b"abc");
/// assert_eq!(h, "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
/// ```
pub fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let digest = hasher.finalize(); // GenericArray<u8, 32>
    hex::encode(digest)
}

/// 便捷封装：直接对字符串文本计算 SHA-256（按 UTF-8 字节哈希）。
///
/// ```
/// let h = rt_49_sha2::sha256_str("abc");
/// assert_eq!(h.len(), 64); // SHA-256 永远是 64 个十六进制字符
/// ```
pub fn sha256_str(text: &str) -> String {
    sha256_hex(text.as_bytes())
}

/// 计算一段字节数据的 SHA-512 摘要，返回小写十六进制字符串。
///
/// 注意代码和 [`sha256_hex`] 几乎一模一样——只是把 `Sha256` 换成了
/// `Sha512`。这正是 [`Digest`](sha2::Digest) trait 的价值：算法可替换，
/// 调用方式不变。SHA-512 的摘要是 64 字节，即 128 个十六进制字符。
///
/// ```
/// let h = rt_49_sha2::sha512_hex(b"abc");
/// assert_eq!(h.len(), 128);
/// ```
pub fn sha512_hex(data: &[u8]) -> String {
    let mut hasher = Sha512::new();
    hasher.update(data);
    let digest = hasher.finalize(); // GenericArray<u8, 64>
    hex::encode(digest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_of_empty_input() {
        // 空输入的 SHA-256 是一个经典常量。
        assert_eq!(
            sha256_hex(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        );
    }

    #[test]
    fn sha256_of_abc_matches_known_vector() {
        assert_eq!(
            sha256_str("abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
        );
    }

    #[test]
    fn sha256_is_deterministic() {
        // 同样的输入永远得到同样的输出。
        assert_eq!(sha256_str("hello"), sha256_str("hello"));
    }

    #[test]
    fn one_bit_change_avalanche() {
        // 输入只差一个字符，输出就完全不同（雪崩效应）。
        assert_ne!(sha256_str("hello"), sha256_str("hellp"));
    }

    #[test]
    fn sha512_has_128_hex_chars() {
        assert_eq!(sha512_hex(b"abc").len(), 128);
    }

    #[test]
    fn update_can_be_called_in_chunks() {
        // 分块喂入和一次性喂入结果相同。
        let mut hasher = Sha256::new();
        hasher.update(b"foo");
        hasher.update(b"bar");
        let chunked = hex::encode(hasher.finalize());
        assert_eq!(chunked, sha256_str("foobar"));
    }
}
