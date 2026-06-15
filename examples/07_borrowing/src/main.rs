//! 共享借用 `&T` 与可变借用 `&mut T`。

/// 可变借用：原地修改调用方的数据。
fn append_exclaim(text: &mut String) {
    text.push('!');
}

/// 共享借用：只读访问。
fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

fn main() {
    let mut message = String::from("hello");
    append_exclaim(&mut message);
    println!("{message}");

    let sentence = String::from("the quick fox");
    // 不可变借用，可以同时存在多个。
    let word = first_word(&sentence);
    println!("first word = {word}, full = {sentence}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mutates_through_reference() {
        let mut text = String::from("ok");
        append_exclaim(&mut text);
        assert_eq!(text, "ok!");
    }

    #[test]
    fn reads_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word(""), "");
    }
}
