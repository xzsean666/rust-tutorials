//! 泛型函数与泛型结构体。

/// 对任何可比较、可复制的类型求最大值。
fn largest<T: PartialOrd + Copy>(items: &[T]) -> Option<T> {
    let mut iter = items.iter();
    let mut max = *iter.next()?;
    for &item in iter {
        if item > max {
            max = item;
        }
    }
    Some(max)
}

#[derive(Debug, PartialEq)]
struct Pair<T> {
    first: T,
    second: T,
}

impl<T: Clone> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Self { first, second }
    }

    /// 返回交换顺序后的新 Pair。
    fn swapped(&self) -> Self {
        Self {
            first: self.second.clone(),
            second: self.first.clone(),
        }
    }
}

fn main() {
    println!("largest int = {:?}", largest(&[3, 7, 2, 9, 4]));
    println!("largest char = {:?}", largest(&['a', 'z', 'm']));

    let pair = Pair::new(1, 2);
    println!("swapped = {:?}", pair.swapped());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_largest() {
        assert_eq!(largest(&[3, 7, 2]), Some(7));
        assert_eq!(largest(&['a', 'c', 'b']), Some('c'));
        assert_eq!(largest::<i32>(&[]), None);
    }

    #[test]
    fn swaps_pair() {
        assert_eq!(Pair::new(1, 2).swapped(), Pair::new(2, 1));
    }
}
