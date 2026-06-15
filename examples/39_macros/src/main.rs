//! 声明宏 macro_rules!:在编译期生成代码。

/// 取两个值中较大者(演示最简单的宏)。
macro_rules! my_max {
    ($a:expr, $b:expr) => {
        if $a >= $b { $a } else { $b }
    };
}

/// 用 `key => value` 字面量构造 HashMap,支持任意个数和结尾逗号。
macro_rules! hashmap {
    ($($key:expr => $value:expr),* $(,)?) => {{
        #[allow(unused_mut)]
        let mut map = std::collections::HashMap::new();
        $(
            map.insert($key, $value);
        )*
        map
    }};
}

fn main() {
    println!("my_max = {}", my_max!(3, 7));

    let scores = hashmap! {
        "alice" => 90,
        "bob" => 85,
    };
    println!("alice = {:?}", scores.get("alice"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn max_macro() {
        assert_eq!(my_max!(3, 7), 7);
        assert_eq!(my_max!(9, 2), 9);
    }

    #[test]
    fn hashmap_macro() {
        let map = hashmap! {
            "a" => 1,
            "b" => 2,
        };
        assert_eq!(map.get("a"), Some(&1));
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn hashmap_macro_empty() {
        let map: std::collections::HashMap<&str, i32> = hashmap! {};
        assert!(map.is_empty());
    }
}
