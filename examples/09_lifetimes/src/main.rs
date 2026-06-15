//! 生命周期（lifetime）：告诉编译器“引用之间的存活关系”。
//!
//! 生命周期标注**不会**改变任何值实际存活多久，它只是给借用检查器
//! （borrow checker）提供信息，让它能在编译期证明“引用永远不会指向
//! 已经被释放的数据”（即避免悬垂引用 dangling reference）。
//!
//! 本示例覆盖：
//! 1. 函数上的显式生命周期标注（`longest`），以及为什么借用检查器需要它；
//! 2. 持有引用的结构体（`Excerpt<'a>`）及其方法；
//! 3. 生命周期省略规则（elision）——为什么 `first_word` 不用写 `'a`；
//! 4. `'static` 生命周期。

/// 返回两个字符串切片中**较长**的那一个。
///
/// 为什么必须写 `<'a>`？因为函数返回一个引用，编译器无法自己判断
/// 这个引用到底借自 `left` 还是 `right`（运行时才知道）。于是我们用
/// 同一个生命周期 `'a` 标注两个入参和返回值，含义是：
/// “返回的引用至少和两个入参中**存活更短**的那个一样久”。
///
/// 调用方因此知道：只要 `left` 和 `right` 都还活着，返回值就是安全的。
fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}

/// 一个“书摘”：它**借用**了别处字符串的一部分，并不拥有数据。
///
/// 因为字段 `part` 是引用，结构体必须声明生命周期参数 `'a`。
/// 它表达的约束是：`Excerpt` 实例**不能比**它所引用的字符串活得更久，
/// 否则 `part` 就会变成悬垂引用。
struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    /// 关联函数：从一段文本切片构造书摘。
    fn new(part: &'a str) -> Self {
        Self { part }
    }

    /// 返回内部引用。
    ///
    /// 这里**用到了生命周期省略规则**：方法只有一个 `&self` 输入引用，
    /// 编译器自动把返回值的生命周期设为和 `&self` 相同，所以我们
    /// 无需手写 `-> &'a str`。
    fn part(&self) -> &str {
        self.part
    }

    /// 打印一句“通知”，并返回 `self.part`。
    ///
    /// 当输入里有 `&self` 时，省略规则规定：返回引用的生命周期跟随 `&self`，
    /// 因此即使有多个引用参数（`announcement`），这里依然不必标注。
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("  注意请看：{announcement}");
        self.part
    }
}

/// 返回第一个单词（到第一个空格为止）。
///
/// 这是**生命周期省略**的经典例子：只有一个输入引用 `s`，编译器自动
/// 把返回引用的生命周期绑定到 `s`。等价于手写 `fn first_word<'a>(s: &'a str) -> &'a str`，
/// 但我们什么都不用写。
fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}

/// 返回一个 `'static` 引用：它指向程序的整个生命周期都存在的数据。
///
/// 字符串字面量（`&'static str`）被直接编入二进制，永远不会被释放，
/// 因此可以安全地从任意位置返回。
fn motto() -> &'static str {
    "安全源于编译期的证明"
}

fn main() {
    // 1. 函数上的显式生命周期
    let a = String::from("a fairly long string");
    let b = String::from("short");
    println!("[longest] 较长的是：{}", longest(&a, &b));

    // 生命周期允许两个入参来自**不同**作用域，只要在调用点都还活着。
    {
        let inner = String::from("x");
        println!("[longest] 跨作用域：{}", longest(&a, &inner));
    }

    // 2. 持有引用的结构体 + 方法
    let novel = String::from("Call me Ishmael. Some years ago I went sailing.");
    let first_sentence = novel.split('.').next().unwrap_or("");
    let excerpt = Excerpt::new(first_sentence);
    println!("[Excerpt] 首句：{}", excerpt.part());
    let part = excerpt.announce_and_return_part("下面是引用的片段");
    println!("[Excerpt] 返回的片段：{part}");

    // 3. 省略规则：first_word 没有任何 'a 标注，照样工作
    let sentence = String::from("hello rust world");
    println!("[first_word] 第一个单词：{}", first_word(&sentence));

    // 4. 'static 生命周期
    println!("[static] 座右铭：{}", motto());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picks_longest() {
        assert_eq!(longest("abcd", "ab"), "abcd");
        assert_eq!(longest("a", "bcd"), "bcd");
    }

    #[test]
    fn longest_prefers_left_on_tie() {
        // 长度相等时返回 left（实现里用了 `>=`）。
        assert_eq!(longest("ab", "cd"), "ab");
    }

    #[test]
    fn excerpt_borrows() {
        let text = String::from("first. second.");
        let excerpt = Excerpt::new(&text[..5]);
        assert_eq!(excerpt.part(), "first");
    }

    #[test]
    fn announce_returns_part() {
        let excerpt = Excerpt::new("hello");
        assert_eq!(excerpt.announce_and_return_part("看这里"), "hello");
    }

    #[test]
    fn first_word_elision_works() {
        assert_eq!(first_word("foo bar"), "foo");
        assert_eq!(first_word("single"), "single");
    }

    #[test]
    fn motto_is_static() {
        let s: &'static str = motto();
        assert!(!s.is_empty());
    }
}
