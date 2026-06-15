//! 闭包与 Fn / FnMut / FnOnce。

/// 接受一个 Fn 闭包并调用它。
fn apply<F: Fn(i32) -> i32>(function: F, value: i32) -> i32 {
    function(value)
}

/// 返回一个捕获了 n 的闭包（impl Fn）。
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |value| value + n
}

/// 返回一个会修改内部状态的 FnMut 闭包。
fn make_counter() -> impl FnMut() -> i32 {
    let mut count = 0;
    move || {
        count += 1;
        count
    }
}

fn main() {
    println!("apply double 5 = {}", apply(|x| x * 2, 5));

    let add_ten = make_adder(10);
    println!("add_ten(3) = {}", add_ten(3));

    let mut counter = make_counter();
    println!("counter: {} {} {}", counter(), counter(), counter());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn applies_closure() {
        assert_eq!(apply(|x| x + 1, 41), 42);
    }

    #[test]
    fn adder_captures_value() {
        let add_five = make_adder(5);
        assert_eq!(add_five(10), 15);
    }

    #[test]
    fn counter_keeps_state() {
        let mut counter = make_counter();
        assert_eq!(counter(), 1);
        assert_eq!(counter(), 2);
        assert_eq!(counter(), 3);
    }
}
