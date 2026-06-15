//! 用 mpsc channel 在线程间传递消息。

use std::sync::mpsc;
use std::thread;

/// 生产者线程发送平方数,主线程收集。
fn squares(count: i32) -> Vec<i32> {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        for value in 1..=count {
            sender.send(value * value).expect("接收端已关闭");
        }
        // sender 在此被 drop,接收端的迭代随之结束。
    });

    // rx 的迭代器会一直收到发送端关闭为止。
    receiver.iter().collect()
}

fn main() {
    println!("squares = {:?}", squares(5));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collects_messages() {
        assert_eq!(squares(4), vec![1, 4, 9, 16]);
    }

    #[test]
    fn zero_produces_nothing() {
        assert_eq!(squares(0), Vec::<i32>::new());
    }
}
