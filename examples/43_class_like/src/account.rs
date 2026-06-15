//! 封装:struct 存字段(默认私有)+ impl 提供方法。
//!
//! 字段不加 `pub` 默认私有,模块外只能通过方法访问,
//! 等价于其它语言把字段设为 private、再提供 getter / setter。

use std::fmt;

use crate::account_trait::Account;

/// 一个银行账户。余额以「分」为单位存储,且保证永不为负 —— 这条不变量
/// 由方法来维护,外部拿不到可变字段,自然无法破坏它。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BankAccount {
    owner: String,
    balance: i64,
}

impl BankAccount {
    /// 关联常量:类似「类的静态常量」。用 `BankAccount::MIN_OPENING` 访问。
    pub const MIN_OPENING: i64 = 100;

    /// 关联函数(无 `self`)= 构造器 / 静态工厂方法。
    /// 返回 `Result`,把「开户金额不足」这种非法状态挡在对象诞生之前。
    pub fn new(owner: &str, opening: i64) -> Result<Self, String> {
        if opening < Self::MIN_OPENING {
            return Err(format!("开户金额至少 {} 分", Self::MIN_OPENING));
        }
        Ok(Self {
            owner: owner.to_string(),
            balance: opening,
        })
    }

    /// `&self`:只读 getter。返回借用,避免不必要的克隆。
    pub fn owner(&self) -> &str {
        &self.owner
    }

    /// `&mut self`:会修改自身的实例方法。
    pub fn deposit(&mut self, amount: i64) {
        self.balance += amount;
    }

    /// `&mut self`:取款时维护「余额不为负」的不变量,违反就返回错误。
    pub fn withdraw(&mut self, amount: i64) -> Result<(), String> {
        if amount > self.balance {
            return Err(format!("余额不足:想取 {amount},仅有 {}", self.balance));
        }
        self.balance -= amount;
        Ok(())
    }
}

/// toString:实现 `Display`,而不是写一个 `to_string` 方法。
/// 实现了 Display 就自动获得 `.to_string()` 和 `{}` 格式化能力。
impl fmt::Display for BankAccount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} 的账户:{:.2} 元",
            self.owner,
            self.balance as f64 / 100.0
        )
    }
}

impl Account for BankAccount {
    fn balance(&self) -> i64 {
        self.balance
    }

    fn kind(&self) -> &str {
        "普通账户"
    }
    // 不重写 summary,直接用 trait 的默认实现。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_low_opening() {
        assert!(BankAccount::new("阿俊", 50).is_err());
    }

    #[test]
    fn deposit_and_withdraw_keep_invariant() {
        let mut acc = BankAccount::new("阿俊", 1_000).unwrap();
        acc.deposit(500);
        assert_eq!(acc.balance(), 1_500);
        assert!(acc.withdraw(2_000).is_err()); // 不变量:不能透支
        assert!(acc.withdraw(1_000).is_ok());
        assert_eq!(acc.balance(), 500);
    }

    #[test]
    fn display_formats_yuan() {
        let acc = BankAccount::new("阿俊", 12_345).unwrap();
        assert_eq!(acc.to_string(), "阿俊 的账户:123.45 元");
    }

    #[test]
    fn default_trait_method_used_by_bank_account() {
        let acc = BankAccount::new("阿俊", 1_000).unwrap();
        assert_eq!(acc.summary(), "[普通账户] 余额 1000 分");
    }
}
