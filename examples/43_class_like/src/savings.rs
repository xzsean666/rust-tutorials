//! 「继承」用组合代替:composition over inheritance。
//!
//! Rust 不能继承 struct。要复用 BankAccount 的能力,就把它作为字段
//! 「包」进来,新增自己的字段/方法,并把需要的调用「委托」过去。

use crate::account::BankAccount;
use crate::account_trait::Account;

/// 储蓄账户:在普通账户之上增加「计息」能力。
pub struct SavingsAccount {
    base: BankAccount, // 组合:拥有一个 BankAccount,而非继承它
    rate_bps: i64,     // 年利率,单位:基点(1% = 100bps)
}

impl SavingsAccount {
    pub fn new(owner: &str, opening: i64, rate_bps: i64) -> Result<Self, String> {
        Ok(Self {
            base: BankAccount::new(owner, opening)?, // 复用父类型的构造器
            rate_bps,
        })
    }

    /// 新增能力:按利率结算一次利息。
    pub fn accrue_interest(&mut self) {
        let interest = self.base.balance() * self.rate_bps / 10_000;
        self.base.deposit(interest);
    }

    /// 委托:把基础能力转发给内部的 BankAccount(等价于调用「父类方法」)。
    pub fn deposit(&mut self, amount: i64) {
        self.base.deposit(amount);
    }
}

// 让「子类型」也满足同一个接口 —— 通过委托实现 trait,
// 于是 SavingsAccount 能和 BankAccount 一起被多态地使用。
impl Account for SavingsAccount {
    fn balance(&self) -> i64 {
        self.base.balance()
    }

    fn kind(&self) -> &str {
        "储蓄账户"
    }

    /// 重写(覆盖)默认方法 —— 类似子类 override 基类方法。
    fn summary(&self) -> String {
        format!(
            "[{}] 余额 {} 分,年化 {}bps",
            self.kind(),
            self.balance(),
            self.rate_bps
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn savings_overrides_summary_and_accrues() {
        let mut savings = SavingsAccount::new("小美", 100_000, 250).unwrap();
        savings.accrue_interest(); // 100000 * 250 / 10000 = 2500
        assert_eq!(savings.balance(), 102_500);
        assert!(savings.summary().contains("储蓄账户"));
        assert!(savings.summary().contains("250bps"));
    }
}
