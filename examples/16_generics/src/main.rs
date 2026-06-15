//! 泛型：用类型参数写出适用于多种类型的函数、结构体和方法。
//!
//! 本例覆盖：
//! - 带 trait bound 的泛型函数（`largest<T: PartialOrd + Copy>`）。
//! - 泛型结构体 `Pair<T>` 及其泛型方法（`new` / `swapped`）。
//! - 多个类型参数的结构体 `Labeled<K, V>` 与 `where` 子句。
//! - 条件实现（conditional impl）：仅当 `T: Display` 时才提供 `describe`。

use std::fmt::Display;

/// 返回切片中最大的元素；空切片返回 `None`。
///
/// `T: PartialOrd + Copy`：`PartialOrd` 让 `>` 可用，`Copy` 让
/// `*item` 能按位复制，从而把元素移出切片而不影响原切片。
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

/// 返回切片中最小的元素；空切片返回 `None`。
///
/// 与 `largest` 同样的约束，只是比较方向相反。
fn smallest<T: PartialOrd + Copy>(items: &[T]) -> Option<T> {
    let mut iter = items.iter();
    let mut min = *iter.next()?;
    for &item in iter {
        if item < min {
            min = item;
        }
    }
    Some(min)
}

/// 同类型的一对值。`T` 是唯一的类型参数，两个字段必须同类型。
#[derive(Debug, PartialEq)]
struct Pair<T> {
    first: T,
    second: T,
}

impl<T: Clone> Pair<T> {
    /// 构造一个新的 `Pair`。
    fn new(first: T, second: T) -> Self {
        Self { first, second }
    }

    /// 返回交换两个字段顺序后的新 `Pair`。
    ///
    /// 需要 `T: Clone`，因为 `&self` 不允许把字段移出借用。
    fn swapped(&self) -> Self {
        Self {
            first: self.second.clone(),
            second: self.first.clone(),
        }
    }
}

/// 条件实现：只有当 `T: PartialOrd` 时，`Pair<T>` 才有 `larger` 方法。
///
/// 这样 `Pair<T>` 对不可比较的 `T` 依然可用，只是不提供这个方法。
impl<T: PartialOrd + Clone> Pair<T> {
    /// 返回两个字段中较大的那个（克隆一份）。
    fn larger(&self) -> T {
        if self.first >= self.second {
            self.first.clone()
        } else {
            self.second.clone()
        }
    }
}

/// 带标签的值：两个类型参数 `K`（标签）和 `V`（数据）可以不同。
#[derive(Debug, PartialEq)]
struct Labeled<K, V> {
    label: K,
    value: V,
}

impl<K, V> Labeled<K, V> {
    /// 构造一个带标签的值。
    fn new(label: K, value: V) -> Self {
        Self { label, value }
    }

    /// 只替换数据部分，返回标签类型不变、数据类型可变的新值。
    ///
    /// 注意返回类型是 `Labeled<K, W>`：方法本身又引入了新的类型参数 `W`。
    fn map_value<W>(self, f: impl FnOnce(V) -> W) -> Labeled<K, W> {
        Labeled {
            label: self.label,
            value: f(self.value),
        }
    }
}

/// 用 `where` 子句表达较长的约束，比写在尖括号里更清晰。
///
/// 仅当标签和数据都能打印时，才提供这个方法。
impl<K, V> Labeled<K, V>
where
    K: Display,
    V: Display,
{
    /// 返回 `"标签: 值"` 形式的字符串。
    fn describe(&self) -> String {
        format!("{}: {}", self.label, self.value)
    }
}

fn main() {
    println!("== 泛型函数 ==");
    println!("largest int  = {:?}", largest(&[3, 7, 2, 9, 4]));
    println!("largest char = {:?}", largest(&['a', 'z', 'm']));
    println!("smallest f64 = {:?}", smallest(&[3.5, 1.2, 8.0]));

    println!("\n== 泛型结构体与方法 ==");
    let pair = Pair::new(1, 2);
    println!("pair       = {pair:?}");
    println!("swapped    = {:?}", pair.swapped());
    println!("larger     = {}", pair.larger());

    println!("\n== 多类型参数与 where 子句 ==");
    let item = Labeled::new("price", 42);
    println!("describe   = {}", item.describe());
    // map_value 把数据从 i32 变成 String，标签类型保持不变。
    let mapped = item.map_value(|v| format!("${v}.00"));
    println!("mapped     = {}", mapped.describe());
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
    fn finds_smallest() {
        assert_eq!(smallest(&[3, 7, 2]), Some(2));
        assert_eq!(smallest(&[3.5, 1.2, 8.0]), Some(1.2));
        assert_eq!(smallest::<i32>(&[]), None);
    }

    #[test]
    fn swaps_pair() {
        assert_eq!(Pair::new(1, 2).swapped(), Pair::new(2, 1));
    }

    #[test]
    fn picks_larger_field() {
        assert_eq!(Pair::new(10, 3).larger(), 10);
        assert_eq!(Pair::new("apple", "banana").larger(), "banana");
    }

    #[test]
    fn maps_value_changes_type() {
        let item = Labeled::new("n", 7);
        let mapped = item.map_value(|v| v.to_string());
        assert_eq!(mapped, Labeled::new("n", "7".to_string()));
    }

    #[test]
    fn describes_with_display() {
        assert_eq!(Labeled::new("price", 42).describe(), "price: 42");
    }
}
