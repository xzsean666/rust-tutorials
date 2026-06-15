//! Rc<T>：单线程下的共享所有权（引用计数）。

use std::rc::Rc;

/// 一个被多处共享的配置。
#[derive(Debug, PartialEq)]
struct Config {
    name: String,
}

fn main() {
    let shared = Rc::new(Config {
        name: "prod".to_string(),
    });
    println!("count after create = {}", Rc::strong_count(&shared));

    // clone 只增加引用计数，不复制底层数据。
    let a = Rc::clone(&shared);
    let b = Rc::clone(&shared);
    println!("count after 2 clones = {}", Rc::strong_count(&shared));
    println!("a.name = {}, b.name = {}", a.name, b.name);

    drop(a);
    drop(b);
    println!("count after drops = {}", Rc::strong_count(&shared));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clone_increases_count() {
        let shared = Rc::new(Config {
            name: "x".to_string(),
        });
        assert_eq!(Rc::strong_count(&shared), 1);
        let clone = Rc::clone(&shared);
        assert_eq!(Rc::strong_count(&shared), 2);
        drop(clone);
        assert_eq!(Rc::strong_count(&shared), 1);
    }

    #[test]
    fn clones_share_data() {
        let shared = Rc::new(Config {
            name: "same".to_string(),
        });
        let clone = Rc::clone(&shared);
        // 指向同一块数据。
        assert!(Rc::ptr_eq(&shared, &clone));
    }
}
