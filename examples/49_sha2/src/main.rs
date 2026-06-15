//! 瘦入口：演示用 `sha2` 计算摘要、用 `hex` 编码，并做一次完整性校验。

use rt_49_sha2::{matches, sha256_str, sha512_hex};

fn main() {
    println!("=== SHA-256 摘要（十六进制）===");
    for text in ["", "abc", "hello", "hellp"] {
        println!("  {:>7?} -> {}", text, sha256_str(text));
    }

    println!("\n=== 雪崩效应：输入差一个字符，输出完全不同 ===");
    println!("  hello -> {}", sha256_str("hello"));
    println!("  hellp -> {}", sha256_str("hellp"));

    println!("\n=== SHA-512（128 个十六进制字符）===");
    println!("  abc -> {}", sha512_hex(b"abc"));

    println!("\n=== 完整性校验 ===");
    let expected = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";
    println!(
        "  matches(b\"abc\", 正确值) = {}",
        matches(b"abc", expected)
    );
    println!(
        "  matches(b\"abd\", 正确值) = {}",
        matches(b"abd", expected)
    );
}
