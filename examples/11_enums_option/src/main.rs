//! 枚举与 Option：用带数据的枚举建模“多选一”，用 `Option<T>` 表达“可能没有值”。
//!
//! 本示例覆盖：
//! - 带数据的变体（结构体式与元组式）。
//! - 为枚举实现 `impl` 方法，并在方法里 `match self`。
//! - `Option<T>` 的常见组合子：`map` / `and_then` / `unwrap_or` / `unwrap_or_else`。
//! - `if let` 与 `let ... else` 两种简写。

/// 几何图形：每个变体携带各自需要的字段。
#[derive(Debug, PartialEq)]
enum Shape {
    /// 圆，由半径决定。
    Circle { radius: f64 },
    /// 矩形，由宽高决定。
    Rectangle { width: f64, height: f64 },
    /// 三角形（元组式变体）：底、高。
    Triangle(f64, f64),
}

impl Shape {
    /// 计算面积——方法内部 `match self`，编译器保证覆盖所有变体。
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle(base, height) => 0.5 * base * height,
        }
    }

    /// 返回变体的中文名字，演示对 `self` 的只读匹配。
    fn name(&self) -> &'static str {
        match self {
            Shape::Circle { .. } => "圆",
            Shape::Rectangle { .. } => "矩形",
            Shape::Triangle(..) => "三角形",
        }
    }
}

/// 一个更贴近业务的枚举：用户界面事件。
#[derive(Debug)]
enum Event {
    /// 点击坐标。
    Click { x: i32, y: i32 },
    /// 按下某个键。
    KeyPress(char),
    /// 粘贴一段文本。
    Paste(String),
    /// 窗口关闭，无附加数据。
    Close,
}

impl Event {
    /// 把事件渲染成一行人类可读的日志。
    fn describe(&self) -> String {
        match self {
            Event::Click { x, y } => format!("在 ({x}, {y}) 点击"),
            Event::KeyPress(c) => format!("按下按键 '{c}'"),
            Event::Paste(text) => format!("粘贴了 {} 个字符", text.chars().count()),
            Event::Close => "关闭窗口".to_string(),
        }
    }
}

/// 用 `Option` 表达“可能没有结果”：找不到偶数时返回 `None`。
fn first_even(values: &[i32]) -> Option<i32> {
    values.iter().copied().find(|value| value % 2 == 0)
}

/// 把字符串解析成正数：解析失败或非正数都返回 `None`。
///
/// 演示 `and_then`：只有上一步是 `Some` 时才继续，否则短路为 `None`。
fn parse_positive(text: &str) -> Option<i32> {
    text.parse::<i32>()
        .ok()
        .and_then(|n| if n > 0 { Some(n) } else { None })
}

/// 取首个偶数的平方；没有偶数则回退到 0。
///
/// 演示 `map`（在 `Some` 内部变换）与 `unwrap_or`（提供默认值）。
fn first_even_squared_or_zero(values: &[i32]) -> i32 {
    first_even(values).map(|n| n * n).unwrap_or(0)
}

fn main() {
    println!("== 枚举 + impl 方法 ==");
    let shapes = [
        Shape::Circle { radius: 1.0 },
        Shape::Rectangle {
            width: 2.0,
            height: 3.0,
        },
        Shape::Triangle(4.0, 3.0),
    ];
    for shape in &shapes {
        println!("{} 面积 = {:.2}", shape.name(), shape.area());
    }

    println!("\n== 业务枚举 Event ==");
    let events = [
        Event::Click { x: 10, y: 20 },
        Event::KeyPress('A'),
        Event::Paste("你好，世界".to_string()),
        Event::Close,
    ];
    for event in &events {
        println!("{}", event.describe());
    }

    println!("\n== Option 与组合子 ==");
    // match：最完整的处理方式。
    match first_even(&[1, 3, 4, 7]) {
        Some(value) => println!("首个偶数 = {value}"),
        None => println!("没有偶数"),
    }

    // if let：只关心 Some 分支时的简写。
    if let Some(n) = parse_positive("42") {
        println!("解析出正数 = {n}");
    }

    // let ... else：拿不到值就提前返回/退出，主流程保持平铺。
    let Some(first) = first_even(&[5, 6, 7]) else {
        println!("不应发生：切片里没有偶数");
        return;
    };
    println!("let-else 取到首个偶数 = {first}");

    // map / unwrap_or：变换后给默认值。
    println!(
        "首个偶数的平方(无则0) = {}",
        first_even_squared_or_zero(&[1, 3, 5])
    );

    // unwrap_or_else：默认值需要计算时用闭包，避免无谓开销。
    let parsed = parse_positive("oops").unwrap_or_else(|| {
        println!("解析失败，使用兜底值");
        -1
    });
    println!("最终值 = {parsed}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rectangle_area() {
        let shape = Shape::Rectangle {
            width: 2.0,
            height: 3.0,
        };
        assert_eq!(shape.area(), 6.0);
    }

    #[test]
    fn circle_area_is_positive() {
        let shape = Shape::Circle { radius: 1.0 };
        assert!((shape.area() - std::f64::consts::PI).abs() < 1e-9);
    }

    #[test]
    fn triangle_area_and_name() {
        let shape = Shape::Triangle(4.0, 3.0);
        assert_eq!(shape.area(), 6.0);
        assert_eq!(shape.name(), "三角形");
    }

    #[test]
    fn finds_first_even() {
        assert_eq!(first_even(&[1, 3, 4, 7]), Some(4));
        assert_eq!(first_even(&[1, 3, 5]), None);
    }

    #[test]
    fn parse_positive_filters_non_positive() {
        assert_eq!(parse_positive("7"), Some(7));
        assert_eq!(parse_positive("0"), None);
        assert_eq!(parse_positive("-3"), None);
        assert_eq!(parse_positive("abc"), None);
    }

    #[test]
    fn squared_or_zero_falls_back() {
        assert_eq!(first_even_squared_or_zero(&[1, 3, 4]), 16);
        assert_eq!(first_even_squared_or_zero(&[1, 3, 5]), 0);
    }

    #[test]
    fn event_describe_counts_chars() {
        let event = Event::Paste("你好".to_string());
        assert_eq!(event.describe(), "粘贴了 2 个字符");
    }
}
