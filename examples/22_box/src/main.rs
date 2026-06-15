//! Box<T>：把数据放到堆上，支持递归类型。

/// 经典的 cons 链表，递归类型必须用 Box 才能确定大小。
#[derive(Debug, PartialEq)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

/// 递归求和。
fn sum(list: &List) -> i32 {
    match list {
        Cons(value, rest) => value + sum(rest),
        Nil => 0,
    }
}

/// 计算链表长度。
fn len(list: &List) -> usize {
    match list {
        Cons(_, rest) => 1 + len(rest),
        Nil => 0,
    }
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list = {list:?}");
    println!("sum = {}", sum(&list));
    println!("len = {}", len(&list));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> List {
        Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))))
    }

    #[test]
    fn sums_list() {
        assert_eq!(sum(&sample()), 6);
        assert_eq!(sum(&Nil), 0);
    }

    #[test]
    fn measures_length() {
        assert_eq!(len(&sample()), 3);
        assert_eq!(len(&Nil), 0);
    }
}
