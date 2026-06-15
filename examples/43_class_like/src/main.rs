//! 运行各模块的小演示。真正的实现都在库 crate(见 `src/lib.rs` 及其模块)。

use rt_43_class_like::{Account, BankAccount, SavingsAccount, print_static, total_balance};

fn main() {
    println!("== 1. 封装:构造器 + 方法维护不变量 ==");
    let mut acc = BankAccount::new("阿俊", 10_000).expect("开户成功");
    acc.deposit(5_000);
    match acc.withdraw(3_000) {
        Ok(()) => println!("  取款成功,余额 {}", acc.balance()),
        Err(e) => println!("  取款失败:{e}"),
    }
    if let Err(e) = acc.withdraw(999_999) {
        println!("  非法取款被拦截:{e}");
    }
    // Display:直接用 {} 打印,等价于其它语言的 toString。
    println!("  Display -> {acc}");

    println!("== 2. 关联常量当作静态常量使用 ==");
    println!("  最低开户金额 = {} 分", BankAccount::MIN_OPENING);

    println!("== 3. 组合代替继承:储蓄账户计息 ==");
    let mut savings = SavingsAccount::new("小美", 100_000, 250).expect("开户成功");
    savings.accrue_interest();
    println!("  计息后:{}", savings.summary());

    println!("== 4. 多态:静态分发 vs 动态分发 ==");
    print_static(&acc);
    print_static(&savings);
    let accounts: Vec<Box<dyn Account>> = vec![Box::new(acc), Box::new(savings)];
    for a in &accounts {
        println!("  dynamic -> {}", a.summary());
    }
    println!("  全部账户余额合计 = {} 分", total_balance(&accounts));
}
