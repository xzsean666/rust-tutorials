//! 生命周期标注：告诉编译器引用之间的存活关系。

/// 返回值的生命周期与两个入参中较短的一致。
fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}

/// 持有引用的结构体必须标注生命周期。
struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    fn new(part: &'a str) -> Self {
        Self { part }
    }

    fn part(&self) -> &str {
        self.part
    }
}

fn main() {
    let a = String::from("long string");
    let b = String::from("short");
    println!("longest = {}", longest(&a, &b));

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap_or("");
    let excerpt = Excerpt::new(first_sentence);
    println!("excerpt = {}", excerpt.part());
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
    fn excerpt_borrows() {
        let text = String::from("first. second.");
        let excerpt = Excerpt::new(&text[..5]);
        assert_eq!(excerpt.part(), "first");
    }
}
