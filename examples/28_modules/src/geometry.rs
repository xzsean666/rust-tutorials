//! geometry 模块:聚合子模块并暴露公共接口。

pub mod area;

/// 公开函数,可被父模块通过 `geometry::describe()` 调用。
pub fn describe() -> &'static str {
    "geometry module"
}
