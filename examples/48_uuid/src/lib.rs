//! 用 `uuid` 这个生态库生成、解析与格式化唯一标识符（UUID）。
//!
//! UUID（Universally Unique Identifier）是一个 128 位的标识符，几乎可以保证
//! 全局唯一，常用来给数据库记录、文件、会话等分配 ID，而不必依赖中心化的
//! 自增计数器。
//!
//! 本章覆盖：
//! - 用 `Uuid::new_v4()` 生成 v4（随机）UUID；
//! - 给业务实体分配一个新鲜的 ID；
//! - 用 `Uuid::parse_str` 把字符串解析回 `Uuid`；
//! - 在带连字符（hyphenated）与无连字符（simple）两种文本形式之间转换。
//!
//! 代码按职责拆分为两个模块：
//! - [`generate`]：生成 UUID 与 `Entity` 实体；
//! - [`parsing`]：解析字符串与格式化输出。

pub mod generate;
pub mod parsing;

pub use generate::{Entity, new_id, new_ids};
pub use parsing::{nil_id, parse_id, to_hyphenated, to_simple};
