//! 演示入口：调用库里拆分好的 `summary` 与 `shape` 模块，跑一遍带标签的输出。

// 注意：调用 trait 方法（如 summarize / area）需要把对应 trait 引入作用域。
use rt_17_traits::{
    Article, Circle, Rectangle, Summary, Tweet, announce, describe_shape, featured, headline,
};

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
