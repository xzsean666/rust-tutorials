//! 摘要相关：`Summary` trait、`Article`/`Tweet` 类型，以及操作 `Summary` 的函数。
//!
//! 本模块演示：
//! - 定义 trait 与“必须实现”的方法；
//! - 默认方法，以及在实现里覆盖默认方法；
//! - 把 trait 作为泛型函数的约束（trait bound）；
//! - 参数位置与返回位置的 `impl Trait`。

/// 一个描述“可被摘要”行为的 trait。
pub trait Summary {
    /// 必须由实现者提供：返回内容的一行摘要。
    fn summarize(&self) -> String;

    /// 默认方法：在摘要后追加提示语。
    ///
    /// 实现者若不覆盖就直接复用这份实现，因此它能调用 `self.summarize()`。
    fn preview(&self) -> String {
        format!("{} (点击查看更多)", self.summarize())
    }

    /// 默认方法：统计摘要里的词数（按空白切分）。
    fn word_count(&self) -> usize {
        self.summarize().split_whitespace().count()
    }
}

/// 文章：拥有标题与作者。
pub struct Article {
    pub title: String,
    pub author: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} - {}", self.title, self.author)
    }
    // 不覆盖 preview / word_count，直接使用默认实现。
}

/// 推文：拥有用户名与正文。
pub struct Tweet {
    pub user: String,
    pub text: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.user, self.text)
    }

    /// 覆盖默认实现：推文本身已足够简短，预览就等于摘要。
    fn preview(&self) -> String {
        self.summarize()
    }
}

/// 用泛型 + trait bound 接受任意实现了 `Summary` 的类型。
///
/// `<T: Summary>` 与参数位置的 `&impl Summary` 等价，但泛型写法可以在多个
/// 参数间共享同一个类型 `T`，表达力更强。
pub fn announce<T: Summary>(item: &T) -> String {
    format!("最新: {}", item.preview())
}

/// 参数位置的 `impl Trait`：是上面泛型写法的“匿名”语法糖，更简洁。
pub fn headline(item: &impl Summary) -> String {
    format!("[{} 词] {}", item.word_count(), item.summarize())
}

/// 返回位置的 `impl Trait`：调用方只知道“这是一个 Summary”，
/// 不必关心具体类型。适合返回闭包或难以书写的复杂类型。
pub fn featured() -> impl Summary {
    Tweet {
        user: "rustlang".to_string(),
        text: "Rust 2024 发布啦".to_string(),
    }
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

    #[test]
    fn default_word_count() {
        let tweet = Tweet {
            user: "u".to_string(),
            text: "hello world".to_string(),
        };
        // summarize 为 "@u: hello world"，按空白切分得 3 个词。
        assert_eq!(tweet.word_count(), 3);
    }

    #[test]
    fn returned_impl_trait_summarizes() {
        let item = featured();
        assert_eq!(item.summarize(), "@rustlang: Rust 2024 发布啦");
    }
}
