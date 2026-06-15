//! trait 定义共享行为，可带默认方法。

trait Summary {
    /// 必须实现的方法。
    fn summarize(&self) -> String;

    /// 默认方法，实现者可以直接复用。
    fn preview(&self) -> String {
        format!("{} (点击查看更多)", self.summarize())
    }
}

struct Article {
    title: String,
    author: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} - {}", self.title, self.author)
    }
}

struct Tweet {
    user: String,
    text: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.user, self.text)
    }

    // 覆盖默认实现。
    fn preview(&self) -> String {
        self.summarize()
    }
}

/// trait bound：接受任何实现了 Summary 的类型。
fn announce(item: &impl Summary) -> String {
    format!("最新: {}", item.preview())
}

fn main() {
    let article = Article {
        title: "Rust 入门".to_string(),
        author: "Alice".to_string(),
    };
    let tweet = Tweet {
        user: "bob".to_string(),
        text: "Rust 真香".to_string(),
    };
    println!("{}", announce(&article));
    println!("{}", announce(&tweet));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_default_preview() {
        let article = Article {
            title: "T".to_string(),
            author: "A".to_string(),
        };
        assert_eq!(article.preview(), "T - A (点击查看更多)");
    }

    #[test]
    fn overrides_preview() {
        let tweet = Tweet {
            user: "u".to_string(),
            text: "hi".to_string(),
        };
        assert_eq!(tweet.preview(), "@u: hi");
    }
}
