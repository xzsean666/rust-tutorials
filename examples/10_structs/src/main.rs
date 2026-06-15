//! 结构体、方法与关联函数。

#[derive(Debug, Clone, PartialEq)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    /// 关联函数（无 self），常用作构造器。
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// 方法（带 &self），只读访问字段。
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }

    /// 返回一个新结构体，不修改自身。
    fn scaled(&self, factor: u32) -> Self {
        Self::new(self.width * factor, self.height * factor)
    }
}

fn main() {
    let rect = Rectangle::new(3, 4);
    println!("{rect:?}");
    println!("area = {}", rect.area());
    println!("is_square = {}", rect.is_square());
    println!("scaled = {:?}", rect.scaled(2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_area() {
        assert_eq!(Rectangle::new(3, 4).area(), 12);
    }

    #[test]
    fn detects_square() {
        assert!(Rectangle::new(5, 5).is_square());
        assert!(!Rectangle::new(5, 6).is_square());
    }

    #[test]
    fn scales_into_new_value() {
        let rect = Rectangle::new(2, 3);
        assert_eq!(rect.scaled(2), Rectangle::new(4, 6));
        // 原值不变。
        assert_eq!(rect, Rectangle::new(2, 3));
    }
}
