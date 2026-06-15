//! 所有权与移动语义。

/// 接收 `String` 的所有权并消费它。
fn consume(owned: String) -> usize {
    owned.len()
}

/// 通过引用借用，不夺走所有权。
fn total_length(parts: &[String]) -> usize {
    parts.iter().map(String::len).sum()
}

fn main() {
    let greeting = String::from("hello");
    // greeting 的所有权被移动进 consume，之后不能再用 greeting。
    let length = consume(greeting);
    println!("consumed length = {length}");

    // 需要保留时可以显式 clone。
    let original = String::from("rust");
    let copy = original.clone();
    println!("original = {original}, copy = {copy}");

    let parts = vec![String::from("rust"), String::from("ocean")];
    // 传引用，parts 仍然归调用方所有。
    println!("total = {}", total_length(&parts));
    println!("still usable = {parts:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consumes_and_measures() {
        assert_eq!(consume(String::from("hello")), 5);
    }

    #[test]
    fn borrows_without_taking() {
        let parts = vec![String::from("ab"), String::from("cde")];
        assert_eq!(total_length(&parts), 5);
        // parts 在借用后仍可访问。
        assert_eq!(parts.len(), 2);
    }
}
