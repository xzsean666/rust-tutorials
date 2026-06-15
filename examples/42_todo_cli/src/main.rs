//! 待办应用的命令行入口:clap 解析 + JSON 文件持久化。

use clap::{Parser, Subcommand};
use rt_42_todo_cli::TodoList;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "todo", about = "一个最小的命令行待办应用")]
struct Cli {
    /// 数据文件路径。
    #[arg(long, default_value = "todo.json")]
    file: PathBuf,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// 新增任务。
    Add { title: String },
    /// 标记完成。
    Done { id: u32 },
    /// 列出全部任务。
    List,
}

/// 从文件加载;文件不存在或损坏时返回空清单。
fn load(path: &PathBuf) -> TodoList {
    fs::read_to_string(path)
        .ok()
        .and_then(|text| TodoList::from_json(&text).ok())
        .unwrap_or_default()
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let mut list = load(&cli.file);

    match cli.command {
        Command::Add { title } => {
            let id = list.add(&title)?;
            println!("已添加 #{id}: {title}");
        }
        Command::Done { id } => {
            list.complete(id)?;
            println!("已完成 #{id}");
        }
        Command::List => {
            if list.tasks().is_empty() {
                println!("(空)");
            }
            for task in list.tasks() {
                let mark = if task.done { "x" } else { " " };
                println!("[{mark}] #{} {}", task.id, task.title);
            }
            println!("未完成: {}", list.pending());
        }
    }

    fs::write(&cli.file, list.to_json()?)?;
    Ok(())
}
