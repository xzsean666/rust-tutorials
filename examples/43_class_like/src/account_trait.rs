//! 接口 / 抽象基类:trait。
//!
//! trait 既能像 interface 规定「必须实现的方法」,
//! 也能像抽象基类提供「默认实现」。

/// 「账户」契约。任何想被当作账户对待的类型都要实现它。
pub trait Account {
    /// 抽象方法:必须实现。
    fn balance(&self) -> i64;
    /// 抽象方法:必须实现。
    fn kind(&self) -> &str;

    /// 默认方法:不重写就用这份实现(类似抽象基类里的具体方法)。
    fn summary(&self) -> String {
        format!("[{}] 余额 {} 分", self.kind(), self.balance())
    }
}
