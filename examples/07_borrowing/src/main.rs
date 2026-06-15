//! 引用与借用：共享借用 `&T` 与可变借用 `&mut T`。
//!
//! 借用让你在 **不转移所有权** 的前提下访问数据。核心规则只有一条：
//! 在任意时刻，要么存在 **任意多个共享借用 `&T`**，要么存在
//! **恰好一个可变借用 `&mut T`**，二者不能同时存在。借用检查器在
//! 编译期强制这条规则，从而在不需要垃圾回收的情况下杜绝数据竞争。

/// 共享借用：只读访问，不获取所有权。
///
/// 参数写成 `&[i32]`（切片）而不是 `&Vec<i32>`，这样数组、`Vec`、
/// 其它切片都能直接传进来，更通用。
fn sum(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

/// 可变借用：原地修改调用方的数据，调用结束后所有权仍归调用方。
fn double_all(numbers: &mut [i32]) {
    for n in numbers.iter_mut() {
        *n *= 2; // 解引用 `*n` 才能写入被借用的值
    }
}

/// 返回借用：返回的引用指向 **传入的** 数据，因此不会悬垂。
///
/// 入参用 `&str` 而非 `&String`：`&String` 会自动“解引用强制转换”
/// 成 `&str`，所以 `&str` 能同时接受字符串字面量和 `String` 的借用。
fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

/// 一个小账户类型，用来演示 `&self`（只读方法）与 `&mut self`（可变方法）。
struct Account {
    balance: i64,
}

impl Account {
    fn new(balance: i64) -> Self {
        Self { balance }
    }

    /// `&self`：只读借用自身，可以被多次、并发地调用。
    fn balance(&self) -> i64 {
        self.balance
    }

    /// `&mut self`：可变借用自身，同一时刻只能有一个这样的借用。
    fn deposit(&mut self, amount: i64) {
        self.balance += amount;
    }
}

fn main() {
    // --- 共享借用：可以同时存在多个 ---
    let data = vec![1, 2, 3, 4];
    let a = &data;
    let b = &data; // 多个共享借用并存，没有问题
    println!("[共享借用] a.len={}, b.sum={}", a.len(), sum(b));
    println!("[共享借用] 原 data 仍可用 = {data:?}");

    // --- 可变借用：原地修改 ---
    let mut scores = vec![10, 20, 30];
    double_all(&mut scores);
    println!("[可变借用] 翻倍后 = {scores:?}");

    // --- 非词法生命周期（NLL）：借用在最后一次使用后即结束 ---
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    println!("[借用范围] first = {first}"); // first 用完了
    v.push(4); // 此处再可变借用 v 是允许的，因为上面的共享借用已结束
    println!("[借用范围] push 之后 = {v:?}");

    // --- 返回借用 & &str/&String ---
    let owned = String::from("the quick fox");
    println!("[返回借用] first_word = {}", first_word(&owned));
    println!("[返回借用] 字面量也行 = {}", first_word("hello world"));

    // --- &self vs &mut self ---
    let mut account = Account::new(100);
    println!("[方法] 初始余额 = {}", account.balance());
    account.deposit(50); // 需要 &mut self
    println!("[方法] 存款后余额 = {}", account.balance());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_borrow_reads() {
        let data = vec![1, 2, 3, 4];
        assert_eq!(sum(&data), 10);
        // 借用之后原数据仍然可用。
        assert_eq!(data.len(), 4);
    }

    #[test]
    fn mutable_borrow_writes() {
        let mut numbers = vec![1, 2, 3];
        double_all(&mut numbers);
        assert_eq!(numbers, vec![2, 4, 6]);
    }

    #[test]
    fn first_word_handles_str_and_string() {
        let owned = String::from("hello world");
        // &String 自动转成 &str。
        assert_eq!(first_word(&owned), "hello");
        assert_eq!(first_word("rust ocean"), "rust");
        assert_eq!(first_word(""), "");
    }

    #[test]
    fn account_self_methods() {
        let mut account = Account::new(100);
        assert_eq!(account.balance(), 100); // &self
        account.deposit(50); // &mut self
        assert_eq!(account.balance(), 150);
    }
}
