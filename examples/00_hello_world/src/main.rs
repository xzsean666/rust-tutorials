fn greeting(language: &str) -> String {
    format!("你好，{language}! 欢迎学习 Rust。")
}

fn main() {
    println!("{}", greeting("Rust"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_greeting() {
        assert_eq!(greeting("Rust"), "你好，Rust! 欢迎学习 Rust。");
    }
}
