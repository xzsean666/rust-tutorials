//! 用 [`rayon`](https://docs.rs/rayon) 做数据并行（data parallelism）。
//!
//! rayon 最迷人的一点：把迭代器链里的 `iter()` 换成 `par_iter()`，几乎零改动
//! 就能利用多核。它内部维护一个工作窃取（work-stealing）线程池，自动把任务
//! 切片、分发、做负载均衡，你不必手写 `thread::spawn` 或管理通道。
//!
//! 之所以「换个名字就行」还能保证结果正确，是因为这些操作是**纯函数**、
//! 归约满足**结合律**：拆开分别算、再把局部结果合并，和顺序计算等价。
//!
//! 本章覆盖：
//! - [`par_sum_of_squares`]：`par_iter().map().sum()` 的并行归约；
//! - [`par_count_primes`]：`into_par_iter().filter().count()` 的 CPU 密集型并行；
//! - [`par_sorted`]：`par_sort_unstable` 的就地并行排序。
//!
//! 代码按职责拆分为两个模块：
//! - [`parallel_map`]：并行 `map` / `filter` 与归约；
//! - [`sorting`]：并行排序。

pub mod parallel_map;
pub mod sorting;

pub use parallel_map::{par_count_primes, par_sum_of_squares};
pub use sorting::par_sorted;
