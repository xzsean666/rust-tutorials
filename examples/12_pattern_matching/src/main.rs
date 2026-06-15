//! match、if let、守卫与解构。

#[derive(Debug)]
enum Command {
    Move { x: i32, y: i32 },
    Write(String),
    Quit,
}

fn describe(command: &Command) -> String {
    match command {
        Command::Move { x, y } => format!("move to ({x}, {y})"),
        Command::Write(text) => format!("write: {text}"),
        Command::Quit => "quit".to_string(),
    }
}

/// 守卫和范围模式。
fn classify(value: i32) -> &'static str {
    match value {
        0 => "zero",
        n if n < 0 => "negative",
        1..=9 => "small",
        _ => "large",
    }
}

fn main() {
    let commands = [
        Command::Move { x: 1, y: 2 },
        Command::Write(String::from("hi")),
        Command::Quit,
    ];
    for command in &commands {
        println!("{}", describe(command));
    }

    // if let 只关心一种模式。
    let maybe = Some(7);
    if let Some(value) = maybe {
        println!("got {value}");
    }

    for value in [-3, 0, 5, 42] {
        println!("{value} => {}", classify(value));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn describes_commands() {
        assert_eq!(describe(&Command::Move { x: 1, y: 2 }), "move to (1, 2)");
        assert_eq!(describe(&Command::Write("hi".into())), "write: hi");
        assert_eq!(describe(&Command::Quit), "quit");
    }

    #[test]
    fn classifies_values() {
        assert_eq!(classify(0), "zero");
        assert_eq!(classify(-1), "negative");
        assert_eq!(classify(5), "small");
        assert_eq!(classify(100), "large");
    }
}
