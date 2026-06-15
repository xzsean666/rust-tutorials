//! Result 与 `?` 运算符。

use std::num::ParseIntError;

/// 把空白分隔的数字求和，任何解析失败都向上传播。
fn parse_sum(input: &str) -> Result<i32, ParseIntError> {
    let mut total = 0;
    for token in input.split_whitespace() {
        total += token.parse::<i32>()?;
    }
    Ok(total)
}

/// 用 unwrap_or 提供默认值。
fn parse_or_zero(token: &str) -> i32 {
    token.parse::<i32>().unwrap_or(0)
}

fn main() {
    match parse_sum("1 2 3") {
        Ok(total) => println!("sum = {total}"),
        Err(error) => println!("error: {error}"),
    }

    match parse_sum("1 x 3") {
        Ok(total) => println!("sum = {total}"),
        Err(error) => println!("error: {error}"),
    }

    println!("parse_or_zero(\"42\") = {}", parse_or_zero("42"));
    println!("parse_or_zero(\"oops\") = {}", parse_or_zero("oops"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_valid_input() {
        assert_eq!(parse_sum("1 2 3"), Ok(6));
        assert_eq!(parse_sum(""), Ok(0));
    }

    #[test]
    fn propagates_error() {
        assert!(parse_sum("1 x 3").is_err());
    }

    #[test]
    fn falls_back_to_zero() {
        assert_eq!(parse_or_zero("42"), 42);
        assert_eq!(parse_or_zero("oops"), 0);
    }
}
