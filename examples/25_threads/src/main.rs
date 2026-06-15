//! 用 thread::spawn 创建线程,用 join 收集结果。

use std::thread;

/// 把每个分片交给一个线程求和,最后汇总。
fn parallel_sum(chunks: Vec<Vec<i32>>) -> i32 {
    let handles: Vec<_> = chunks
        .into_iter()
        .map(|chunk| thread::spawn(move || chunk.iter().sum::<i32>()))
        .collect();

    handles
        .into_iter()
        .map(|handle| handle.join().expect("线程 panic"))
        .sum()
}

fn main() {
    let chunks = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    println!("parallel sum = {}", parallel_sum(chunks));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_in_parallel() {
        let chunks = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(parallel_sum(chunks), 21);
    }

    #[test]
    fn handles_empty() {
        assert_eq!(parallel_sum(vec![]), 0);
    }
}
