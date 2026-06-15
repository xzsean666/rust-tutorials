//! RefCell<T>：把借用检查从编译期推迟到运行期（内部可变性）。

use std::cell::RefCell;
use std::rc::Rc;

/// 即使只持有 &self，也能通过 RefCell 修改内部状态。
struct Logger {
    messages: RefCell<Vec<String>>,
}

impl Logger {
    fn new() -> Self {
        Self {
            messages: RefCell::new(Vec::new()),
        }
    }

    fn log(&self, message: &str) {
        self.messages.borrow_mut().push(message.to_string());
    }

    fn count(&self) -> usize {
        self.messages.borrow().len()
    }
}

fn main() {
    // Rc<RefCell<T>>：多个所有者共享同一份可变数据。
    let logger = Rc::new(Logger::new());
    let clone = Rc::clone(&logger);

    logger.log("started");
    clone.log("working");
    logger.log("done");

    println!("logged {} messages", logger.count());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logs_through_shared_reference() {
        let logger = Rc::new(Logger::new());
        let clone = Rc::clone(&logger);
        logger.log("a");
        clone.log("b");
        assert_eq!(logger.count(), 2);
    }

    #[test]
    fn interior_mutability_without_mut() {
        let logger = Logger::new(); // 注意：不是 mut
        logger.log("x");
        assert_eq!(logger.count(), 1);
    }
}
