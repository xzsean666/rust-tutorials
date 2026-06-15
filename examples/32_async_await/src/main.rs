//! async/await 基础:用 tokio 运行异步函数。

use std::time::Duration;

/// async 函数返回一个 Future,`.await` 时才真正执行。
async fn fetch(id: u32) -> String {
    // 模拟一次异步 IO。
    tokio::time::sleep(Duration::from_millis(10)).await;
    format!("data-{id}")
}

/// 顺序 await 两个异步操作。
async fn fetch_both() -> String {
    let first = fetch(1).await;
    let second = fetch(2).await;
    format!("{first},{second}")
}

#[tokio::main]
async fn main() {
    println!("{}", fetch_both().await);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn fetches_one() {
        assert_eq!(fetch(7).await, "data-7");
    }

    #[tokio::test]
    async fn fetches_both() {
        assert_eq!(fetch_both().await, "data-1,data-2");
    }
}
