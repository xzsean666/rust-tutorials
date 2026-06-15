//! 瘦入口：调用库里的并行函数，打印结果，并粗略对比一次串行/并行耗时。
//!
//! 注意：耗时只用于直观感受，不做任何断言——真实加速比取决于机器核数、
//! 数据规模与系统负载。

use std::time::Instant;

use rt_50_rayon::{par_count_primes, par_sorted, par_sum_of_squares};

fn main() {
    println!("=== rayon 数据并行 ===\n");

    // 1. 并行归约：平方和
    let nums: Vec<u64> = (1..=1_000).collect();
    println!("[平方和] 1..=1000 的平方和 = {}", par_sum_of_squares(&nums));

    // 2. 并行排序
    let v = vec![5, 3, 8, 1, 9, 2, 7, 4, 6, 0];
    let sorted = par_sorted(v.clone());
    println!("[排序]   {v:?} -> {sorted:?}");

    // 3. CPU 密集型并行：素数计数（顺手做一次粗略计时对比）
    let upto = 2_000_000;

    let t0 = Instant::now();
    let seq = (0..upto)
        .filter(|&n| {
            if n < 2 {
                return false;
            }
            let mut d = 2;
            while d * d <= n {
                if n % d == 0 {
                    return false;
                }
                d += 1;
            }
            true
        })
        .count();
    let seq_dur = t0.elapsed();

    let t1 = Instant::now();
    let par = par_count_primes(upto);
    let par_dur = t1.elapsed();

    println!("\n[素数]   [0, {upto}) 内素数个数 = {par}");
    println!("         顺序耗时 ≈ {seq_dur:?}");
    println!("         并行耗时 ≈ {par_dur:?}");
    println!("         （两者结果一致：{}）", seq == par);
    println!("\n提示：数据越大、单个任务越重，并行收益越明显；数据很小时反受调度开销拖累。");
}
