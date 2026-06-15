//! trait（特征）：定义一组类型可以共享的行为，类似其他语言的“接口”。
//!
//! 本章覆盖：
//! - 定义 trait 与“必须实现”的方法；
//! - 为多个类型实现同一个 trait；
//! - 默认方法，以及在实现里覆盖默认方法；
//! - 把 trait 作为泛型函数的约束（trait bound）；
//! - 参数位置与返回位置的 `impl Trait`；
//! - 为自定义类型实现标准库 trait（`std::fmt::Display`）。
//!
//! 本章聚焦“静态分发”（编译期单态化）；运行时多态用的 `dyn` trait 对象
//! 留到下一章。

use std::fmt;

/// 一个描述“可被摘要”行为的 trait。
trait Summary {
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
struct Article {
    title: String,
    author: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} - {}", self.title, self.author)
    }
    // 不覆盖 preview / word_count，直接使用默认实现。
}

/// 推文：拥有用户名与正文。
struct Tweet {
    user: String,
    text: String,
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
fn announce<T: Summary>(item: &T) -> String {
    format!("最新: {}", item.preview())
}

/// 参数位置的 `impl Trait`：是上面泛型写法的“匿名”语法糖，更简洁。
fn headline(item: &impl Summary) -> String {
    format!("[{} 词] {}", item.word_count(), item.summarize())
}

/// 返回位置的 `impl Trait`：调用方只知道“这是一个 Summary”，
/// 不必关心具体类型。适合返回闭包或难以书写的复杂类型。
fn featured() -> impl Summary {
    Tweet {
        user: "rustlang".to_string(),
        text: "Rust 2024 发布啦".to_string(),
    }
}

/// 一个带“面积”行为的 trait，演示用 trait bound 写通用算法。
trait Shape {
    /// 返回图形面积。
    fn area(&self) -> f64;

    /// 默认方法：名称，默认未知。
    fn name(&self) -> &str {
        "图形"
    }
}

/// 圆形。
struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn name(&self) -> &str {
        "圆形"
    }
}

/// 矩形。
struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn name(&self) -> &str {
        "矩形"
    }
}

/// 为自定义类型实现标准库 trait `Display`，让它能被 `{}` 直接打印。
impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}×{} 的矩形", self.width, self.height)
    }
}

/// 泛型函数 + trait bound：对任意 `Shape` 给出一句描述。
fn describe_shape<S: Shape>(shape: &S) -> String {
    format!("{} 的面积是 {:.2}", shape.name(), shape.area())
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

    println!("== trait bound 与默认方法 ==");
    println!("{}", announce(&article)); // 用默认 preview
    println!("{}", announce(&tweet)); // 用覆盖后的 preview
    println!("{}", headline(&article)); // impl Trait 参数 + word_count

    println!("\n== 返回位置的 impl Trait ==");
    let pick = featured();
    println!("{}", pick.summarize());

    println!("\n== Shape 与 Display ==");
    let circle = Circle { radius: 2.0 };
    let rect = Rectangle {
        width: 3.0,
        height: 4.0,
    };
    println!("{}", describe_shape(&circle));
    println!("{}", describe_shape(&rect));
    println!("Display: {rect}"); // 直接用 {} 打印
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
    fn shape_area_and_name() {
        let rect = Rectangle {
            width: 3.0,
            height: 4.0,
        };
        assert_eq!(rect.name(), "矩形");
        assert!((rect.area() - 12.0).abs() < 1e-9);
    }

    #[test]
    fn display_for_rectangle() {
        let rect = Rectangle {
            width: 3.0,
            height: 4.0,
        };
        assert_eq!(format!("{rect}"), "3×4 的矩形");
    }

    #[test]
    fn returned_impl_trait_summarizes() {
        let item = featured();
        assert_eq!(item.summarize(), "@rustlang: Rust 2024 发布啦");
    }
}
