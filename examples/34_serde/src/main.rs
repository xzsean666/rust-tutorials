//! serde + serde_json:结构体与 JSON 互转。

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Task {
    id: u32,
    title: String,
    done: bool,
}

fn to_json(task: &Task) -> serde_json::Result<String> {
    serde_json::to_string(task)
}

fn from_json(text: &str) -> serde_json::Result<Task> {
    serde_json::from_str(text)
}

fn main() {
    let task = Task {
        id: 1,
        title: "学习 serde".to_string(),
        done: false,
    };
    let json = to_json(&task).expect("序列化失败");
    println!("json = {json}");

    let parsed = from_json(&json).expect("反序列化失败");
    println!("parsed = {parsed:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_to_json() {
        let task = Task {
            id: 1,
            title: "t".to_string(),
            done: true,
        };
        assert_eq!(
            to_json(&task).unwrap(),
            r#"{"id":1,"title":"t","done":true}"#
        );
    }

    #[test]
    fn round_trips() {
        let task = Task {
            id: 9,
            title: "round".to_string(),
            done: false,
        };
        let json = to_json(&task).unwrap();
        assert_eq!(from_json(&json).unwrap(), task);
    }

    #[test]
    fn rejects_invalid_json() {
        assert!(from_json("not json").is_err());
    }
}
