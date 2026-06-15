//! 一个小演示：调用本 crate 暴露的随机功能并打印带标签的输出。
//!
//! 这些函数底层都用到了第三方包 `rand`，但 `main` 完全感知不到——
//! 这正是「把依赖封装进库、上层只管调用」的好处。

use rt_44_rand::{generate_password, roll_die, roll_n, sum_n};

fn main() {
    println!("=== rand：引用第三方包演示 ===\n");

    println!("掷一次骰子: {}", roll_die());
    println!("连掷 5 次:   {:?}", roll_n(5));
    println!("3 颗骰子之和: {}", sum_n(3));

    println!();
    println!("随机密码(8 位):  {}", generate_password(8));
    println!("随机密码(16 位): {}", generate_password(16));
}
