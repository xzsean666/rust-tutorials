//! 用 rusqlite(内置 SQLite)做基本的增查改。

use rusqlite::{Connection, Result};

#[derive(Debug, PartialEq)]
struct Task {
    id: i64,
    title: String,
    done: bool,
}

/// 建表。
fn setup(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE task (
            id    INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            done  INTEGER NOT NULL DEFAULT 0
        )",
        [],
    )?;
    Ok(())
}

/// 插入一条任务,返回自增 id。
fn add_task(conn: &Connection, title: &str) -> Result<i64> {
    conn.execute("INSERT INTO task (title) VALUES (?1)", [title])?;
    Ok(conn.last_insert_rowid())
}

/// 把任务标记为完成。
fn complete(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("UPDATE task SET done = 1 WHERE id = ?1", [id])?;
    Ok(())
}

/// 按 id 顺序列出所有任务。
fn list_tasks(conn: &Connection) -> Result<Vec<Task>> {
    let mut stmt = conn.prepare("SELECT id, title, done FROM task ORDER BY id")?;
    let rows = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            done: row.get::<_, i64>(2)? != 0,
        })
    })?;
    rows.collect()
}

fn main() -> Result<()> {
    // 内存数据库:进程结束即销毁,适合演示与测试。
    let conn = Connection::open_in_memory()?;
    setup(&conn)?;

    let first = add_task(&conn, "学习 SQLite")?;
    add_task(&conn, "写测试")?;
    complete(&conn, first)?;

    for task in list_tasks(&conn)? {
        println!("{task:?}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seeded() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        setup(&conn).unwrap();
        conn
    }

    #[test]
    fn adds_and_lists() {
        let conn = seeded();
        add_task(&conn, "a").unwrap();
        add_task(&conn, "b").unwrap();
        let tasks = list_tasks(&conn).unwrap();
        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].title, "a");
        assert!(!tasks[0].done);
    }

    #[test]
    fn completes_task() {
        let conn = seeded();
        let id = add_task(&conn, "todo").unwrap();
        complete(&conn, id).unwrap();
        let tasks = list_tasks(&conn).unwrap();
        assert!(tasks[0].done);
    }
}
