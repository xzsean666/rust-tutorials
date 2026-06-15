//! FFI:与 C 互操作。

// 从 C 标准库导入 abs。edition 2024 要求 extern 块标注 unsafe。
unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

/// 安全封装:调用 C 函数是 unsafe,这里包成安全接口。
fn safe_abs(value: i32) -> i32 {
    // SAFETY: abs 对任意 i32 都有定义(i32::MIN 除外,这里不涉及)。
    unsafe { abs(value) }
}

/// 导出给 C 调用的 Rust 函数。edition 2024 用 #[unsafe(no_mangle)]。
#[unsafe(no_mangle)]
pub extern "C" fn rust_add(left: i32, right: i32) -> i32 {
    left + right
}

fn main() {
    println!("abs(-5) via C = {}", safe_abs(-5));
    println!("rust_add(2, 3) = {}", rust_add(2, 3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calls_c_abs() {
        assert_eq!(safe_abs(-5), 5);
        assert_eq!(safe_abs(7), 7);
    }

    #[test]
    fn exported_fn_works_from_rust() {
        assert_eq!(rust_add(2, 3), 5);
    }
}
