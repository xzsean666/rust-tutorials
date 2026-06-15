//! 并发任务:join! 同时等待,spawn 后台执行。

use std::time::Duration;

async fn work(id: u32, delay_ms: u64) -> u32 {
    tokio::time::sleep(Duration::from_millis(delay_ms)).await;
    id * 10
}

/// join! 让多个 Future 并发推进,而不是顺序等待。
async fn run_concurrently() -> Vec<u32> {
    let (first, second, third) = tokio::join!(work(1, 30), work(2, 10), work(3, 20));
    vec![first, second, third]
}

/// spawn 把任务交给运行时调度,返回 JoinHandle。
async fn spawn_sum() -> u32 {
    let mut handles = Vec::new();
    for id in 1..=3 {
        handles.push(tokio::spawn(work(id, 5)));
    }
    let mut total = 0;
    for handle in handles {
        total += handle.await.expect("任务 panic");
    }
    total
}

#[tokio::main]
async fn main() {
    println!("concurrent = {:?}", run_concurrently().await);
    println!("spawned sum = {}", spawn_sum().await);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn joins_in_order() {
        // join! 按参数顺序返回结果,与完成先后无关。
        assert_eq!(run_concurrently().await, vec![10, 20, 30]);
    }

    #[tokio::test]
    async fn spawns_and_sums() {
        assert_eq!(spawn_sum().await, 60);
    }
}
