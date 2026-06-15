//! 切片：对连续序列的借用视图。

/// `&[i32]` 同时接受数组、`Vec` 和切片。
fn sum(values: &[i32]) -> i32 {
    values.iter().sum()
}

/// 返回去掉首尾元素后的子切片。
fn middle(values: &[i32]) -> &[i32] {
    if values.len() < 2 {
        &[]
    } else {
        &values[1..values.len() - 1]
    }
}

/// 字符串切片 `&str` 指向原字符串的一段。
fn first_word(text: &str) -> &str {
    match text.find(' ') {
        Some(index) => &text[..index],
        None => text,
    }
}

fn main() {
    let numbers = [1, 2, 3, 4, 5];
    println!("sum = {}", sum(&numbers));
    println!("middle = {:?}", middle(&numbers));
    println!("first word = {}", first_word("hello rust world"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_any_slice() {
        assert_eq!(sum(&[1, 2, 3]), 6);
        let owned = vec![10, 20];
        assert_eq!(sum(&owned), 30);
    }

    #[test]
    fn takes_middle() {
        assert_eq!(middle(&[1, 2, 3, 4]), &[2, 3]);
        assert_eq!(middle(&[1]), &[] as &[i32]);
    }

    #[test]
    fn slices_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("single"), "single");
    }
}
