//! 一个小演示：调用本 crate 暴露的正则功能并打印带标签的输出。
//!
//! 这些函数底层都用到了第三方包 `regex`，`main` 只管调用、不关心实现——
//! 这正是「把依赖封装进库、上层只管调用」的好处。

use rt_46_regex::{find_numbers, is_email, mask_phones, parse_date};

fn main() {
    println!("=== regex：处理文本演示 ===\n");

    println!("--- 匹配与查找 ---");
    println!(
        "alice@example.com 是邮箱吗? {}",
        is_email("alice@example.com")
    );
    println!("not-an-email 是邮箱吗?     {}", is_email("not-an-email"));
    println!("提取数字: {:?}", find_numbers("买了 3 本书，共 128 元"));

    println!();
    println!("--- 捕获组与替换 ---");
    match parse_date("2026-06-15") {
        Some(d) => println!("解析日期: {}年{}月{}日", d.year, d.month, d.day),
        None => println!("解析日期: 格式不对"),
    }
    println!("手机号打码: {}", mask_phones("联系电话 13812345678"));
}
