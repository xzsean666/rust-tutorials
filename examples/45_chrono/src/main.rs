//! 演示 `rt_45_chrono` 库的用法，带标注的输出。

use chrono::Local;
use rt_45_chrono::{add_days, days_between, format_cn, format_iso, parse_date, sub_days};

fn main() {
    // 1. 解析字符串为日期。
    let date = parse_date("2026-06-15").expect("日期格式应当合法");
    println!("解析结果        : {date}");

    // 2. 两种格式化。
    println!("ISO 格式        : {}", format_iso(date));
    println!("中文格式        : {}", format_cn(date));

    // 3. 日期加减。
    let later = add_days(date, 10);
    let earlier = sub_days(date, 7);
    println!("10 天后         : {}", format_iso(later));
    println!("7 天前          : {}", format_iso(earlier));

    // 4. 求差值。
    println!("两日期相差天数  : {} 天", days_between(date, later));

    // 5. 当前日期（注意：依赖系统时钟，每次运行都会变，所以不写进测试）。
    let today = Local::now().date_naive();
    println!("今天            : {}", format_cn(today));
    println!("距 2026-06-15   : {} 天", days_between(date, today));
}
