//! 自定义错误类型：实现 Display、Error 和 From。

use std::fmt;

#[derive(Debug, PartialEq)]
enum ParseError {
    Empty,
    NotANumber(String),
    NotPositive(i64),
}

impl fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Empty => write!(formatter, "输入为空"),
            ParseError::NotANumber(token) => write!(formatter, "不是数字: {token}"),
            ParseError::NotPositive(value) => write!(formatter, "必须为正数: {value}"),
        }
    }
}

impl std::error::Error for ParseError {}

/// 解析一个正整数，失败时返回带语义的自定义错误。
fn parse_positive(input: &str) -> Result<u32, ParseError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(ParseError::Empty);
    }
    let value: i64 = trimmed
        .parse()
        .map_err(|_| ParseError::NotANumber(trimmed.to_string()))?;
    if value <= 0 {
        return Err(ParseError::NotPositive(value));
    }
    Ok(value as u32)
}

fn main() {
    for input in ["42", "", "abc", "-5"] {
        match parse_positive(input) {
            Ok(value) => println!("{input:?} => {value}"),
            Err(error) => println!("{input:?} => 错误: {error}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_positive() {
        assert_eq!(parse_positive("42"), Ok(42));
        assert_eq!(parse_positive("  7 "), Ok(7));
    }

    #[test]
    fn rejects_bad_input() {
        assert_eq!(parse_positive(""), Err(ParseError::Empty));
        assert_eq!(
            parse_positive("abc"),
            Err(ParseError::NotANumber("abc".to_string()))
        );
        assert_eq!(parse_positive("-5"), Err(ParseError::NotPositive(-5)));
    }

    #[test]
    fn error_displays_message() {
        assert_eq!(ParseError::Empty.to_string(), "输入为空");
    }
}
