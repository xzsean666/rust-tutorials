//! 多态:同一段逻辑作用于不同类型。

use crate::account_trait::Account;

/// 静态分发(泛型):编译期为每个具体类型生成专门代码,零运行时开销。
pub fn print_static(acc: &impl Account) {
    println!("  static  -> {}", acc.summary());
}

/// 动态分发(trait 对象):运行期通过虚表调用,代价是一次间接跳转,
/// 好处是能把「不同的具体类型」装进同一个集合里。
pub fn total_balance(accounts: &[Box<dyn Account>]) -> i64 {
    accounts.iter().map(|a| a.balance()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::account::BankAccount;
    use crate::savings::SavingsAccount;

    #[test]
    fn polymorphism_over_trait_objects() {
        let accounts: Vec<Box<dyn Account>> = vec![
            Box::new(BankAccount::new("阿俊", 1_000).unwrap()),
            Box::new(SavingsAccount::new("小美", 2_000, 100).unwrap()),
        ];
        assert_eq!(total_balance(&accounts), 3_000);
    }
}
