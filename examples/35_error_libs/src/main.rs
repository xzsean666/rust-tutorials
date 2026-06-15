//! thiserror 定义错误类型,anyhow 在应用层聚合错误。

use thiserror::Error;

/// thiserror 自动生成 Display 和 Error 实现。
#[derive(Error, Debug, PartialEq)]
enum ConfigError {
    #[error("配置为空")]
    Empty,
    #[error("端口不是数字: {0}")]
    NotANumber(String),
    #[error("端口超出范围: {0}")]
    OutOfRange(u32),
}

/// 库层:返回精确的自定义错误。
fn parse_port(input: &str) -> Result<u16, ConfigError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(ConfigError::Empty);
    }
    let value: u32 = trimmed
        .parse()
        .map_err(|_| ConfigError::NotANumber(trimmed.to_string()))?;
    if value > u16::MAX as u32 {
        return Err(ConfigError::OutOfRange(value));
    }
    Ok(value as u16)
}

/// 应用层:用 anyhow::Result 聚合不同来源的错误,`?` 自动转换。
fn load_config(input: &str) -> anyhow::Result<u16> {
    let port = parse_port(input)?;
    Ok(port)
}

fn main() -> anyhow::Result<()> {
    let port = load_config("8080")?;
    println!("port = {port}");
    println!("error demo: {}", parse_port("abc").unwrap_err());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_port() {
        assert_eq!(parse_port("8080"), Ok(8080));
    }

    #[test]
    fn reports_specific_errors() {
        assert_eq!(parse_port(""), Err(ConfigError::Empty));
        assert_eq!(
            parse_port("abc"),
            Err(ConfigError::NotANumber("abc".to_string()))
        );
        assert_eq!(parse_port("70000"), Err(ConfigError::OutOfRange(70000)));
    }

    #[test]
    fn anyhow_wraps_error() {
        assert!(load_config("abc").is_err());
        assert_eq!(load_config("443").unwrap(), 443);
    }
}
