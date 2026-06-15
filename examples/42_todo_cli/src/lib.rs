//! 待办清单核心逻辑:综合结构体、枚举、集合、错误处理与序列化。

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// 操作待办清单时可能出现的错误。
#[derive(Debug, Error, PartialEq)]
pub enum TodoError {
    #[error("找不到任务 #{0}")]
    NotFound(u32),
    #[error("标题不能为空")]
    EmptyTitle,
}

/// 单条任务。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub done: bool,
}

/// 待办清单:持有任务集合并分配自增 id。
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TodoList {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TodoList {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    /// 新增任务,返回分配的 id;空标题报错。
    pub fn add(&mut self, title: &str) -> Result<u32, TodoError> {
        let title = title.trim();
        if title.is_empty() {
            return Err(TodoError::EmptyTitle);
        }
        let id = self.next_id;
        self.next_id += 1;
        self.tasks.push(Task {
            id,
            title: title.to_string(),
            done: false,
        });
        Ok(id)
    }

    /// 把指定任务标记为完成。
    pub fn complete(&mut self, id: u32) -> Result<(), TodoError> {
        let task = self
            .tasks
            .iter_mut()
            .find(|task| task.id == id)
            .ok_or(TodoError::NotFound(id))?;
        task.done = true;
        Ok(())
    }

    /// 删除指定任务。
    pub fn remove(&mut self, id: u32) -> Result<(), TodoError> {
        let before = self.tasks.len();
        self.tasks.retain(|task| task.id != id);
        if self.tasks.len() == before {
            Err(TodoError::NotFound(id))
        } else {
            Ok(())
        }
    }

    /// 未完成任务数量。
    pub fn pending(&self) -> usize {
        self.tasks.iter().filter(|task| !task.done).count()
    }

    pub fn tasks(&self) -> &[Task] {
        &self.tasks
    }

    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(text: &str) -> serde_json::Result<Self> {
        serde_json::from_str(text)
    }
}

impl Default for TodoList {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_and_assigns_ids() {
        let mut list = TodoList::new();
        assert_eq!(list.add("a").unwrap(), 1);
        assert_eq!(list.add("b").unwrap(), 2);
        assert_eq!(list.tasks().len(), 2);
    }

    #[test]
    fn rejects_empty_title() {
        let mut list = TodoList::new();
        assert_eq!(list.add("   "), Err(TodoError::EmptyTitle));
    }

    #[test]
    fn completes_and_counts_pending() {
        let mut list = TodoList::new();
        let id = list.add("task").unwrap();
        list.add("other").unwrap();
        assert_eq!(list.pending(), 2);
        list.complete(id).unwrap();
        assert_eq!(list.pending(), 1);
    }

    #[test]
    fn errors_on_missing_id() {
        let mut list = TodoList::new();
        assert_eq!(list.complete(99), Err(TodoError::NotFound(99)));
        assert_eq!(list.remove(99), Err(TodoError::NotFound(99)));
    }

    #[test]
    fn removes_task() {
        let mut list = TodoList::new();
        let id = list.add("temp").unwrap();
        list.remove(id).unwrap();
        assert!(list.tasks().is_empty());
    }

    #[test]
    fn json_round_trip() {
        let mut list = TodoList::new();
        list.add("persist me").unwrap();
        list.complete(1).unwrap();
        let json = list.to_json().unwrap();
        assert_eq!(TodoList::from_json(&json).unwrap(), list);
    }
}
