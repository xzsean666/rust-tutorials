//! 校验数据完整性：把数据重新哈希，再和给定的期望摘要对比。
//!
//! 典型场景：你从网上下载了一个文件，页面同时给出它的 SHA-256 值。
//! 把下载到的内容重新算一遍哈希，和页面给的值比一比，相同就说明
//! 文件完整、没被篡改或损坏。

use crate::hashing::sha256_hex;

/// 校验 `data` 的 SHA-256 是否等于 `expected_hex`（不区分大小写）。
///
/// 期望值在不同地方可能写成大写或小写十六进制，所以这里两边都转成
/// 小写再比较，避免“内容对了但大小写不同”导致误判。
///
/// ```
/// let data = b"abc";
/// // 大写期望值同样能通过校验。
/// assert!(rt_49_sha2::matches(
///     data,
///     "BA7816BF8F01CFEA414140DE5DAE2223B00361A396177A9CB410FF61F20015AD",
/// ));
/// assert!(!rt_49_sha2::matches(data, "deadbeef"));
/// ```
///
/// 安全提示：这里用普通的字符串 `==` 比较，**不是常数时间**的——
/// 它一旦发现某个字符不同就提前返回。对抗计时攻击的场景（例如校验
/// 消息认证码 MAC）应改用常数时间比较（如 `subtle` crate 的
/// `ConstantTimeEq`）。本章只做完整性校验教学，普通比较已经够用。
pub fn matches(data: &[u8], expected_hex: &str) -> bool {
    sha256_hex(data) == expected_hex.to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    const ABC_SHA256: &str = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";

    #[test]
    fn matches_correct_lowercase_hash() {
        assert!(matches(b"abc", ABC_SHA256));
    }

    #[test]
    fn matches_is_case_insensitive() {
        assert!(matches(b"abc", &ABC_SHA256.to_uppercase()));
    }

    #[test]
    fn rejects_wrong_hash() {
        assert!(!matches(b"abc", "deadbeef"));
    }

    #[test]
    fn rejects_when_data_tampered() {
        // 期望值是 "abc" 的哈希，但数据被改成了 "abd"，校验应失败。
        assert!(!matches(b"abd", ABC_SHA256));
    }
}
