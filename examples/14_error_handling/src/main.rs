//! 标准库错误处理：`Result<T, E>`、`?` 运算符与各种组合子。
//!
//! 本章只用标准库，演示「可恢复错误」的处理机制：
//! - `Result<T, E>` 与 `?` 运算符（提前返回 Err）；
//! - 错误传播，以及用 `map_err` 转换错误类型；
//! - 解析字符串（`str::parse` 产生 `ParseIntError`）；
//! - 组合子：`map` / `map_err` / `unwrap_or` / `ok`；
//! - `Option` 与 `Result` 互转：`ok_or`；
//! - 返回 `Box<dyn Error>`，让 `?` 把不同错误类型「装箱」到一起。
//!
//! 自定义错误「类型」是下一章（15_custom_errors）的内容，这里聚焦
//! 标准库提供的机制，不刻意定义庞大的错误枚举。

use std::error::Error;
use std::num::ParseIntError;

/// 把以空白分隔的数字求和，任何解析失败都用 `?` 向上传播。
///
/// `?` 等价于「成功就取出值，失败就 `return Err(...)`」，
/// 因此这里的错误类型必须与 `str::parse` 产生的 `ParseIntError` 一致。
fn parse_and_sum(input: &str) -> Result<i32, ParseIntError> {
    let mut total = 0;
    for token in input.split_whitespace() {
        total += token.parse::<i32>()?; // 失败就提前返回 Err
    }
    Ok(total)
}

/// 用 `map_err` 把底层的 `ParseIntError` 转换成更友好的字符串错误。
///
/// `map_err` 只在 `Err` 时调用闭包，`Ok` 原样保留。这样调用方拿到的
/// 错误信息里带上了出错的具体 token，便于定位问题。
fn parse_port(token: &str) -> Result<u16, String> {
    token
        .parse::<u16>()
        .map_err(|e| format!("无效端口 \"{token}\"：{e}"))
}

/// 用 `unwrap_or` 在出错时提供默认值，函数本身不再返回 `Result`。
fn parse_or_zero(token: &str) -> i32 {
    token.parse::<i32>().unwrap_or(0)
}

/// 用 `map` 在成功分支上做变换：解析后翻倍，失败则保持 Err。
fn parse_doubled(token: &str) -> Result<i32, ParseIntError> {
    token.parse::<i32>().map(|n| n * 2)
}

/// 从切片取第一个元素，用 `ok_or` 把 `Option` 转成 `Result`。
///
/// `Option` 表达「有没有值」，`Result` 表达「成功还是失败并带原因」。
/// `ok_or` 在 `None` 时填入给定的错误，于是后续可以用 `?` 传播。
fn first_number(tokens: &[&str]) -> Result<i32, String> {
    let first = tokens.first().ok_or_else(|| "列表为空".to_string())?;
    first
        .parse::<i32>()
        .map_err(|e| format!("首元素无法解析：{e}"))
}

/// 返回 `Box<dyn Error>`，让 `?` 把「不同类型」的错误统一装箱。
///
/// 这里先后可能产生 `ParseIntError`（解析失败）和我们自己的字符串错误
/// （端口为 0）。它们类型不同，但都实现了 `Error`，因此可以被 `?`
/// 自动转换成 `Box<dyn Error>`，无需手写 `match`。
fn parse_config(line: &str) -> Result<(String, u16), Box<dyn Error>> {
    let (host, port_str) = line.split_once(':').ok_or("配置格式应为 host:port")?; // &str 也能转成 Box<dyn Error>
    let port: u16 = port_str.trim().parse()?; // ParseIntError -> Box<dyn Error>
    if port == 0 {
        return Err("端口不能为 0".into());
    }
    Ok((host.trim().to_string(), port))
}

fn main() {
    // --- ? 运算符与错误传播 ---
    println!("== parse_and_sum ==");
    match parse_and_sum("1 2 3") {
        Ok(total) => println!("  Ok: 求和 = {total}"),
        Err(e) => println!("  Err: {e}"),
    }
    match parse_and_sum("1 x 3") {
        Ok(total) => println!("  Ok: 求和 = {total}"),
        Err(e) => println!("  Err: {e}"),
    }

    // --- map_err 转换错误信息 ---
    println!("== parse_port (map_err) ==");
    println!("  {:?}", parse_port("8080"));
    println!("  {:?}", parse_port("99999")); // 超出 u16 范围

    // --- map 在成功分支上变换 ---
    println!("== parse_doubled (map) ==");
    println!("  {:?}", parse_doubled("21"));
    println!("  {:?}", parse_doubled("oops"));

    // --- unwrap_or 提供默认值 ---
    println!("== parse_or_zero (unwrap_or) ==");
    println!("  \"42\" -> {}", parse_or_zero("42"));
    println!("  \"oops\" -> {}", parse_or_zero("oops"));

    // --- ok：把 Result 丢弃错误后变成 Option ---
    println!("== Result::ok ==");
    let maybe: Option<i32> = "7".parse::<i32>().ok();
    println!("  \"7\".parse().ok() = {maybe:?}");
    println!("  \"no\".parse().ok() = {:?}", "no".parse::<i32>().ok());

    // --- ok_or：Option -> Result ---
    println!("== first_number (ok_or) ==");
    println!("  {:?}", first_number(&["10", "20"]));
    println!("  {:?}", first_number(&[]));

    // --- Box<dyn Error>：统一不同错误类型 ---
    println!("== parse_config (Box<dyn Error>) ==");
    match parse_config("localhost:8080") {
        Ok((host, port)) => println!("  Ok: host={host} port={port}"),
        Err(e) => println!("  Err: {e}"),
    }
    match parse_config("localhost:0") {
        Ok((host, port)) => println!("  Ok: host={host} port={port}"),
        Err(e) => println!("  Err: {e}"),
    }
    match parse_config("badformat") {
        Ok((host, port)) => println!("  Ok: host={host} port={port}"),
        Err(e) => println!("  Err: {e}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_valid_input() {
        assert_eq!(parse_and_sum("1 2 3"), Ok(6));
        assert_eq!(parse_and_sum(""), Ok(0));
    }

    #[test]
    fn propagates_error() {
        assert!(parse_and_sum("1 x 3").is_err());
    }

    #[test]
    fn map_err_adds_context() {
        assert_eq!(parse_port("8080"), Ok(8080));
        let err = parse_port("99999").unwrap_err();
        assert!(err.contains("99999"));
    }

    #[test]
    fn map_doubles_on_success() {
        assert_eq!(parse_doubled("21"), Ok(42));
        assert!(parse_doubled("x").is_err());
    }

    #[test]
    fn falls_back_to_zero() {
        assert_eq!(parse_or_zero("42"), 42);
        assert_eq!(parse_or_zero("oops"), 0);
    }

    #[test]
    fn ok_or_converts_option() {
        assert_eq!(first_number(&["10", "20"]), Ok(10));
        assert!(first_number(&[]).is_err());
    }

    #[test]
    fn boxed_error_unifies_types() {
        assert_eq!(
            parse_config("localhost:8080").unwrap(),
            ("localhost".to_string(), 8080)
        );
        assert!(parse_config("localhost:0").is_err()); // 端口为 0
        assert!(parse_config("badformat").is_err()); // 缺少冒号
    }
}
