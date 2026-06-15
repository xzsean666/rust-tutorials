//! 并行的 `map` / `filter` / 归约：把迭代器链里的 `iter()` 换成 `par_iter()`。
//!
//! rayon 的核心承诺是「几乎零改动」：原本写 `nums.iter().map(...).sum()`，
//! 只需把 `iter()` 改成 `par_iter()`，rayon 的工作窃取（work-stealing）线程池
//! 就会把任务切片、分发到多个核心上跑。结果依旧正确，因为这些运算是
//! 纯函数 + 满足结合律（associative）的归约——拆开求和再合并，和顺序求和等价。

use rayon::prelude::*;

/// 并行计算「每个元素平方之和」。
///
/// 顺序版本是 `nums.iter().map(|&n| n * n).sum()`，这里只把 `iter` 换成
/// `par_iter`。`map` 是纯函数、`sum` 满足结合律，所以无论 rayon 怎样切分、
/// 用多少线程，结果都和顺序求和完全一致——可以在测试里断言精确值。
///
/// ```
/// use rt_50_rayon::par_sum_of_squares;
/// assert_eq!(par_sum_of_squares(&[1, 2, 3]), 14);
/// ```
pub fn par_sum_of_squares(nums: &[u64]) -> u64 {
    nums.par_iter().map(|&n| n * n).sum()
}

/// 判断 `n` 是否为素数（朴素试除法）。
///
/// 这是一个纯 CPU 的小任务，单个不算重，但对每个数都做一遍、数量一大，
/// 总开销就足以让并行化获益。
fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n % 2 == 0 {
        return n == 2;
    }
    let mut d = 3;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 2;
    }
    true
}

/// 并行统计 `[0, upto)` 区间内素数的个数。
///
/// `(0..upto)` 是一个区间，`into_par_iter()` 把它变成并行迭代器；每个数独立
/// 做一次 `is_prime` 判断，互不依赖——这正是数据并行（data parallelism）最
/// 理想的场景：CPU 密集、任务之间没有共享可变状态。`filter` + `count` 是
/// 可结合的归约，所以并行结果与顺序结果相同。
///
/// 例如 `[0, 100)` 内有 25 个素数。
///
/// ```
/// use rt_50_rayon::par_count_primes;
/// assert_eq!(par_count_primes(100), 25);
/// ```
pub fn par_count_primes(upto: u64) -> usize {
    (0..upto).into_par_iter().filter(|&n| is_prime(n)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_of_squares_matches_sequential() {
        let nums: Vec<u64> = (1..=1000).collect();
        // 顺序参考值：1²+2²+…+n² = n(n+1)(2n+1)/6
        let n = 1000u64;
        let expected = n * (n + 1) * (2 * n + 1) / 6;
        assert_eq!(par_sum_of_squares(&nums), expected);
        // 也与顺序迭代器逐位比较，确认并行结果完全一致。
        let seq: u64 = nums.iter().map(|&x| x * x).sum();
        assert_eq!(par_sum_of_squares(&nums), seq);
    }

    #[test]
    fn sum_of_squares_small_and_empty() {
        assert_eq!(par_sum_of_squares(&[]), 0);
        assert_eq!(par_sum_of_squares(&[1, 2, 3]), 14);
    }

    #[test]
    fn count_primes_known_values() {
        // 经典基准：100 以内 25 个、1000 以内 168 个素数。
        assert_eq!(par_count_primes(100), 25);
        assert_eq!(par_count_primes(1000), 168);
    }

    #[test]
    fn count_primes_matches_sequential() {
        let upto = 5000;
        let seq = (0..upto).filter(|&n| is_prime(n)).count();
        assert_eq!(par_count_primes(upto), seq);
    }

    #[test]
    fn is_prime_edge_cases() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(9));
        assert!(is_prime(97));
    }
}
