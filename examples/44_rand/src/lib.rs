//! 用 `rand` 这个第三方包演示「如何引用并使用生态库」。
//!
//! 本 crate 把功能拆成几个模块，方便对照阅读：
//! - `dice`：用范围随机数模拟掷骰子。
//! - `password`：从字符集里随机挑字符，生成随机密码。
//!
//! 所有随机能力都来自 crates.io 上的 `rand` 包（见 `Cargo.toml` 的
//! `[dependencies]`）。这正是「引用第三方包」最典型的样子：声明依赖、
//! `use` 进来、调用它的 API。

pub mod dice;
pub mod password;

// 重新导出常用函数，调用方可以直接 `use rt_44_rand::roll_die;`。
pub use dice::{roll_die, roll_n, sum_n};
pub use password::{CHARSET, generate_password};
