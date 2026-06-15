//! trait（特征）：定义一组类型可以共享的行为，类似其他语言的“接口”。
//!
//! 本章覆盖：
//! - 定义 trait 与“必须实现”的方法；
//! - 为多个类型实现同一个 trait；
//! - 默认方法，以及在实现里覆盖默认方法；
//! - 把 trait 作为泛型函数的约束（trait bound）；
//! - 参数位置与返回位置的 `impl Trait`；
//! - 为自定义类型实现标准库 trait（`std::fmt::Display`）。
//!
//! 本章聚焦“静态分发”（编译期单态化）；运行时多态用的 `dyn` trait 对象
//! 留到下一章。
//!
//! 代码按职责拆分为两个模块：
//! - [`summary`]：`Summary` trait、`Article`/`Tweet` 及操作它们的函数；
//! - [`shape`]：`Shape` trait、`Circle`/`Rectangle`、`Display` 实现及相关函数。

pub mod shape;
pub mod summary;

pub use shape::{Circle, Rectangle, Shape, describe_shape};
pub use summary::{Article, Summary, Tweet, announce, featured, headline};
