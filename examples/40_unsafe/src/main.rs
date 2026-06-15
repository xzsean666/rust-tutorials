//! unsafe:在安全抽象内部使用裸指针等不安全操作。

/// 经典例子:借用检查器无法证明两个可变切片不重叠,
/// 用裸指针 + unsafe 实现,但对外暴露完全安全的接口。
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    assert!(mid <= len, "mid 超出长度");

    // SAFETY: mid <= len,两个子切片不重叠,且都落在原切片范围内。
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

/// 通过裸指针修改值(演示解引用裸指针是 unsafe 操作)。
fn double_in_place(value: &mut i32) {
    let ptr: *mut i32 = value;
    // SAFETY: ptr 来自一个有效的可变引用,非空且对齐。
    unsafe {
        *ptr *= 2;
    }
}

fn main() {
    let mut data = [1, 2, 3, 4, 5];
    let (left, right) = split_at_mut(&mut data, 2);
    println!("left = {left:?}, right = {right:?}");

    let mut value = 21;
    double_in_place(&mut value);
    println!("doubled = {value}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_in_two() {
        let mut data = [1, 2, 3, 4, 5];
        let (left, right) = split_at_mut(&mut data, 2);
        assert_eq!(left, &mut [1, 2]);
        assert_eq!(right, &mut [3, 4, 5]);
    }

    #[test]
    fn doubles_value() {
        let mut value = 21;
        double_in_place(&mut value);
        assert_eq!(value, 42);
    }
}
