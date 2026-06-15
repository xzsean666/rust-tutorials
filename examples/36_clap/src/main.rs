//! clap derive:声明式命令行参数解析。

use clap::Parser;

#[derive(Parser, Debug, PartialEq)]
#[command(name = "greet", about = "打招呼的示例 CLI")]
struct Args {
    /// 要问候的名字。
    #[arg(short, long)]
    name: String,

    /// 重复次数。
    #[arg(short, long, default_value_t = 1)]
    times: u32,
}

fn render(args: &Args) -> String {
    (0..args.times)
        .map(|_| format!("Hello, {}!", args.name))
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() {
    let args = Args::parse();
    println!("{}", render(&args));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_args() {
        let args = Args::try_parse_from(["greet", "--name", "Rust", "--times", "2"]).unwrap();
        assert_eq!(
            args,
            Args {
                name: "Rust".to_string(),
                times: 2,
            }
        );
    }

    #[test]
    fn uses_default_times() {
        let args = Args::try_parse_from(["greet", "--name", "Rust"]).unwrap();
        assert_eq!(args.times, 1);
    }

    #[test]
    fn renders_repeated_greeting() {
        let args = Args {
            name: "Rust".to_string(),
            times: 2,
        };
        assert_eq!(render(&args), "Hello, Rust!\nHello, Rust!");
    }

    #[test]
    fn requires_name() {
        assert!(Args::try_parse_from(["greet"]).is_err());
    }
}
