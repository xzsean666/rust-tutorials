//! Rust 里没有 `class` 关键字,但「类」的能力都能用 struct + impl + trait 表达。
//!
//! 本 crate 按其它语言里熟悉的面向对象概念,把代码拆成几个模块逐一对照:
//! - `account`:封装 + 构造器 + 关联常量 + Display(等价 toString)。
//! - `account_trait`:接口 / 抽象基类(trait + 默认方法)。
//! - `savings`:用「组合 + 委托」代替继承。
//! - `polymorphism`:静态分发(泛型)与动态分发(`dyn`)。

pub mod account;
pub mod account_trait;
pub mod polymorphism;
pub mod savings;

// 重新导出常用类型,调用方可以直接 `use rt_43_class_like::BankAccount`。
pub use account::BankAccount;
pub use account_trait::Account;
pub use polymorphism::{print_static, total_balance};
pub use savings::SavingsAccount;
