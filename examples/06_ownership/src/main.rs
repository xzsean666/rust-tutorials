//! 所有权与移动语义。
//!
//! Rust 没有垃圾回收（GC），靠所有权在编译期决定何时释放内存。三条核心规则：
//! 1. 每个值有且只有一个所有者；
//! 2. 同一时刻只有一个所有者；
//! 3. 所有者离开作用域时，值会被自动 `drop`（释放）。
//!
//! 本示例围绕这三条规则，演示移动、归还、clone 代价、Copy 类型、
//! 作用域触发的 drop，以及用引用避免移动等典型场景。

/// 把 `String` 的所有权移动进函数并消费它。
///
/// 调用后，调用方手中的原变量就失效了——它的所有权已经交给了 `owned`。
fn consume(owned: String) -> usize {
    owned.len()
    // owned 在这里离开作用域并被 drop，其堆内存被释放。
}

/// 接收所有权，做一些处理后再把所有权“归还”给调用方。
///
/// 这是只有所有权语义时常见的写法：拿进来、改一改、还回去。
fn append_excl(mut owned: String) -> String {
    owned.push('!');
    owned
}

/// 通过引用借用，不夺走所有权。
///
/// 调用方在调用后仍然完整持有自己的数据。
fn total_length(parts: &[String]) -> usize {
    parts.iter().map(String::len).sum()
}

/// 接收实现了 `Copy` 的标量类型。
///
/// `i64` 是 `Copy`，传参时按位复制，调用方的原变量依然可用——不会被移动。
fn double(n: i64) -> i64 {
    n * 2
}

/// 一个带自定义 `Drop` 的小结构体，用来直观看到“值在何时被释放”。
struct Guard {
    name: String,
}

impl Guard {
    fn new(name: &str) -> Self {
        println!("  [Guard] 创建 {name}");
        Self {
            name: name.to_string(),
        }
    }
}

impl Drop for Guard {
    /// 当 `Guard` 离开作用域时，Rust 自动调用这里。
    fn drop(&mut self) {
        println!("  [Guard] 释放 {}", self.name);
    }
}

fn main() {
    println!("== 1. 函数调用导致移动 ==");
    let greeting = String::from("hello");
    let length = consume(greeting);
    // 此处不能再用 greeting，它的所有权已经移动进了 consume。
    println!("被消费的长度 = {length}");

    println!("\n== 2. 把所有权归还给调用方 ==");
    let text = String::from("rust");
    let text = append_excl(text); // 移动进去，再用返回值重新绑定。
    println!("归还后的值 = {text}");

    println!("\n== 3. clone 的代价 ==");
    let original = String::from("ocean");
    let copy = original.clone(); // 深拷贝：在堆上复制了一份新数据。
    println!("original = {original}, copy = {copy}");

    println!("\n== 4. Copy 类型 vs 移动类型 ==");
    let n = 21_i64;
    let doubled = double(n); // n 是 Copy，按位复制，下一行仍可用 n。
    println!("n = {n}, doubled = {doubled}");

    println!("\n== 5. 传引用以避免移动 ==");
    let parts = vec![String::from("rust"), String::from("ocean")];
    println!("总长度 = {}", total_length(&parts));
    println!("parts 仍可用 = {parts:?}");

    println!("\n== 6. 作用域与 drop 的顺序 ==");
    let _outer = Guard::new("outer");
    {
        let _inner = Guard::new("inner");
        println!("  内层作用域结束前……");
    } // inner 在这里被 drop
    println!("  外层作用域继续运行……");
    // outer 在 main 结束时被 drop（注意：drop 顺序与创建顺序相反）。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consumes_and_measures() {
        assert_eq!(consume(String::from("hello")), 5);
    }

    #[test]
    fn returns_ownership_back() {
        let s = append_excl(String::from("hi"));
        assert_eq!(s, "hi!");
    }

    #[test]
    fn clone_is_independent() {
        let original = String::from("rust");
        let mut copy = original.clone();
        copy.push_str("acean");
        // 修改 clone 不影响原值，二者是独立的数据。
        assert_eq!(original, "rust");
        assert_eq!(copy, "rustacean");
    }

    #[test]
    fn copy_leaves_original_usable() {
        let n = 10_i64;
        let d = double(n);
        // n 是 Copy 类型，传参后依然可用。
        assert_eq!(n, 10);
        assert_eq!(d, 20);
    }

    #[test]
    fn borrows_without_taking() {
        let parts = vec![String::from("ab"), String::from("cde")];
        assert_eq!(total_length(&parts), 5);
        // parts 在借用后仍可访问。
        assert_eq!(parts.len(), 2);
    }
}
