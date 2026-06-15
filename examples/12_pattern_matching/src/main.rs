//! 模式匹配全景：在 `match`、`if let`、`while let`、`let else` 中
//! 对枚举、结构体、元组解构，并使用守卫、绑定 `@`、范围、或模式
//! 以及 `_` / `..` 通配符。
//!
//! 模式匹配把“判断分支”和“取出内部数据”合并成一步，
//! 编译器还会做穷尽性检查，帮你堵住漏掉的情况。

/// 一个简单的消息枚举，演示三种变体形态：
/// 类单元、元组式、结构体式。
#[derive(Debug)]
enum Message {
    /// 退出，无负载。
    Quit,
    /// 移动到某个坐标，结构体式变体。
    Move { x: i32, y: i32 },
    /// 写入一段文本，元组式变体。
    Write(String),
    /// RGB 颜色，元组式变体，演示多字段解构与守卫。
    Color(u8, u8, u8),
}

/// 平面上的点，用来演示结构体解构与元组解构。
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

/// 对消息分类：综合使用结构体解构、绑定 `@`、守卫和或模式。
fn describe(message: &Message) -> String {
    match message {
        Message::Quit => "退出".to_string(),
        // 结构体式变体解构，并对解构出的字段做条件判断（守卫）。
        Message::Move { x: 0, y: 0 } => "回到原点".to_string(),
        Message::Move { x, y } => format!("移动到 ({x}, {y})"),
        // 元组式变体直接绑定内部值。
        Message::Write(text) => format!("写入：{text}"),
        // 守卫 + 或模式：纯黑或纯白特殊处理。
        Message::Color(0, 0, 0) | Message::Color(255, 255, 255) => "极端的黑或白".to_string(),
        // 绑定 @：把整段值绑到名字上，同时用范围限定红色分量。
        Message::Color(r @ 200..=255, g, b) => format!("偏红的颜色 (r={r}, g={g}, b={b})"),
        Message::Color(r, g, b) => format!("颜色 (r={r}, g={g}, b={b})"),
    }
}

/// 守卫和范围模式：把整数归类到不同档位。
fn classify(value: i32) -> &'static str {
    match value {
        0 => "零",
        n if n < 0 => "负数",
        1..=9 => "个位数",
        10..=99 => "两位数",
        _ => "很大的数",
    }
}

/// 元组解构 + 通配符：判断点落在坐标系的什么位置。
fn locate(point: &Point) -> &'static str {
    match (point.x, point.y) {
        (0, 0) => "原点",
        (_, 0) => "x 轴上",
        (0, _) => "y 轴上",
        // 嵌套模式：两个分量同号即在第一或第三象限。
        (x, y) if (x > 0) == (y > 0) => "第一或第三象限",
        _ => "第二或第四象限",
    }
}

/// 嵌套模式 + `..`：只关心元组的首尾，中间一律忽略。
fn first_and_last(triple: (i32, i32, i32)) -> (i32, i32) {
    let (first, .., last) = triple;
    (first, last)
}

fn main() {
    println!("== match 解构枚举 ==");
    let messages = [
        Message::Quit,
        Message::Move { x: 0, y: 0 },
        Message::Move { x: 3, y: 4 },
        Message::Write(String::from("你好")),
        Message::Color(0, 0, 0),
        Message::Color(220, 30, 30),
        Message::Color(10, 20, 30),
    ];
    for message in &messages {
        println!("{:?} => {}", message, describe(message));
    }

    println!("\n== 元组解构定位点 ==");
    for point in [
        Point { x: 0, y: 0 },
        Point { x: 5, y: 0 },
        Point { x: -2, y: 3 },
    ] {
        println!("{point:?} => {}", locate(&point));
    }

    println!("\n== 守卫与范围分类 ==");
    for value in [-3, 0, 7, 42, 1000] {
        println!("{value} => {}", classify(value));
    }

    println!("\n== if let 只关心一种模式 ==");
    let maybe = Some(7);
    if let Some(value) = maybe {
        println!("拿到了 {value}");
    }

    println!("\n== let else 提前返回 ==");
    // let else：模式不匹配时走 else 分支（这里直接给默认值）。
    let Some(parsed) = "42".parse::<i32>().ok() else {
        unreachable!("字面量一定能解析");
    };
    println!("解析得到 {parsed}");

    println!("\n== while let 弹空栈 ==");
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("弹出 {top}");
    }

    println!("\n== .. 忽略中间元素 ==");
    let (first, last) = first_and_last((10, 20, 30));
    println!("首 {first}，尾 {last}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn describes_messages() {
        assert_eq!(describe(&Message::Quit), "退出");
        assert_eq!(describe(&Message::Move { x: 0, y: 0 }), "回到原点");
        assert_eq!(describe(&Message::Move { x: 3, y: 4 }), "移动到 (3, 4)");
        assert_eq!(describe(&Message::Write("你好".into())), "写入：你好");
    }

    #[test]
    fn describes_colors_with_guards_and_ranges() {
        assert_eq!(describe(&Message::Color(0, 0, 0)), "极端的黑或白");
        assert_eq!(describe(&Message::Color(255, 255, 255)), "极端的黑或白");
        assert_eq!(
            describe(&Message::Color(220, 30, 30)),
            "偏红的颜色 (r=220, g=30, b=30)"
        );
        assert_eq!(
            describe(&Message::Color(10, 20, 30)),
            "颜色 (r=10, g=20, b=30)"
        );
    }

    #[test]
    fn classifies_values() {
        assert_eq!(classify(0), "零");
        assert_eq!(classify(-1), "负数");
        assert_eq!(classify(5), "个位数");
        assert_eq!(classify(42), "两位数");
        assert_eq!(classify(1000), "很大的数");
    }

    #[test]
    fn locates_points() {
        assert_eq!(locate(&Point { x: 0, y: 0 }), "原点");
        assert_eq!(locate(&Point { x: 5, y: 0 }), "x 轴上");
        assert_eq!(locate(&Point { x: 0, y: 9 }), "y 轴上");
        assert_eq!(locate(&Point { x: 2, y: 3 }), "第一或第三象限");
        assert_eq!(locate(&Point { x: -2, y: 3 }), "第二或第四象限");
    }

    #[test]
    fn ignores_middle_with_rest_pattern() {
        assert_eq!(first_and_last((10, 20, 30)), (10, 30));
        assert_eq!(first_and_last((-1, 0, 1)), (-1, 1));
    }
}
