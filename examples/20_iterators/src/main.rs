//! 迭代器适配器：map / filter / collect / fold。

/// map + sum：每个元素平方后求和。
fn sum_of_squares(values: &[i32]) -> i32 {
    values.iter().map(|value| value * value).sum()
}

/// filter + collect：取出所有偶数。
fn evens(values: &[i32]) -> Vec<i32> {
    values
        .iter()
        .copied()
        .filter(|value| value % 2 == 0)
        .collect()
}

/// max_by_key：返回最长的单词。
fn longest_word(text: &str) -> Option<&str> {
    text.split_whitespace().max_by_key(|word| word.len())
}

/// fold：一次遍历同时算出最小值和最大值。
fn min_max(values: &[i32]) -> Option<(i32, i32)> {
    values.iter().fold(None, |acc, &value| match acc {
        None => Some((value, value)),
        Some((min, max)) => Some((min.min(value), max.max(value))),
    })
}

fn main() {
    let numbers = [1, 2, 3, 4, 5];
    println!("sum of squares = {}", sum_of_squares(&numbers));
    println!("evens = {:?}", evens(&numbers));
    println!("longest = {:?}", longest_word("the quick brown fox"));
    println!("min_max = {:?}", min_max(&numbers));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn squares_and_sums() {
        assert_eq!(sum_of_squares(&[1, 2, 3]), 14);
    }

    #[test]
    fn filters_evens() {
        assert_eq!(evens(&[1, 2, 3, 4]), vec![2, 4]);
    }

    #[test]
    fn finds_longest_word() {
        assert_eq!(longest_word("a bb ccc"), Some("ccc"));
        assert_eq!(longest_word(""), None);
    }

    #[test]
    fn folds_min_max() {
        assert_eq!(min_max(&[3, 1, 4, 1, 5]), Some((1, 5)));
        assert_eq!(min_max(&[]), None);
    }
}
