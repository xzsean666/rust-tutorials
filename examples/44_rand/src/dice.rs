//! 掷骰子：用 `rand` 的「范围随机数」模拟一个六面骰。

use rand::Rng;

/// 掷一次六面骰，返回 `1..=6` 之间的点数。
///
/// 关键 API 是 `rng.random_range(1..=6)`：
/// - `rand::rng()` 取得一个线程本地的随机数生成器（RNG）。
/// - `random_range` 接受一个区间，这里 `1..=6` 是「闭区间」，两端都能取到。
///
/// 注意 0.9 版用的是 `rng()` 与 `random_range`；旧教程里的
/// `thread_rng()` / `gen_range` 已被弃用。
pub fn roll_die() -> u8 {
    let mut rng = rand::rng();
    rng.random_range(1..=6)
}

/// 连续掷 `n` 次骰子，把每次点数收集成一个 `Vec<u8>`。
pub fn roll_n(n: usize) -> Vec<u8> {
    (0..n).map(|_| roll_die()).collect()
}

/// 掷 `n` 次骰子并返回点数之和，常用于桌游里的「投掷多颗骰子」。
pub fn sum_n(n: usize) -> u32 {
    roll_n(n).iter().map(|&p| u32::from(p)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roll_die_in_range() {
        // 随机值无法断言具体数字，只能断言「性质」：点数始终在 1..=6。
        for _ in 0..1000 {
            let p = roll_die();
            assert!((1..=6).contains(&p), "点数越界: {p}");
        }
    }

    #[test]
    fn roll_n_length_and_range() {
        let rolls = roll_n(50);
        assert_eq!(rolls.len(), 50);
        assert!(rolls.iter().all(|&p| (1..=6).contains(&p)));
    }

    #[test]
    fn sum_n_within_bounds() {
        // 掷 3 颗骰子，和必然落在 3..=18 之间。
        for _ in 0..200 {
            let s = sum_n(3);
            assert!((3..=18).contains(&s), "和越界: {s}");
        }
    }
}
