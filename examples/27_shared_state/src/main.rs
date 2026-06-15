//! 用 Arc<Mutex<T>> 在多个线程间共享可变状态。

use std::sync::{Arc, Mutex};
use std::thread;

/// 多个线程各自累加同一个计数器。
fn concurrent_count(threads: usize, per_thread: usize) -> usize {
    let counter = Arc::new(Mutex::new(0usize));

    let handles: Vec<_> = (0..threads)
        .map(|_| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..per_thread {
                    let mut value = counter.lock().expect("锁被毒化");
                    *value += 1;
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("线程 panic");
    }

    let total = *counter.lock().expect("锁被毒化");
    total
}

fn main() {
    let total = concurrent_count(4, 1000);
    println!("total = {total}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_without_races() {
        assert_eq!(concurrent_count(4, 1000), 4000);
    }

    #[test]
    fn zero_threads() {
        assert_eq!(concurrent_count(0, 100), 0);
    }
}
