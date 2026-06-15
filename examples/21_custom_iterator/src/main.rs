//! 通过实现 Iterator trait 创建自定义迭代器。

/// 从 1 数到 max（含）的计数器。
struct Counter {
    current: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Self {
        Self { current: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.max {
            self.current += 1;
            Some(self.current)
        } else {
            None
        }
    }
}

fn main() {
    // 自定义迭代器自动获得所有适配器方法。
    let total: u32 = Counter::new(5).sum();
    println!("sum 1..=5 = {total}");

    let doubled: Vec<u32> = Counter::new(3).map(|value| value * 2).collect();
    println!("doubled = {doubled:?}");

    for value in Counter::new(3) {
        println!("count = {value}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yields_sequence() {
        let values: Vec<u32> = Counter::new(3).collect();
        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn empty_when_zero() {
        let values: Vec<u32> = Counter::new(0).collect();
        assert!(values.is_empty());
    }

    #[test]
    fn composes_with_adapters() {
        let total: u32 = Counter::new(4).filter(|value| value % 2 == 0).sum();
        assert_eq!(total, 6);
    }
}
