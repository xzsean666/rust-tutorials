//! 演示入口：调用库里拆分好的 `generate` 与 `parsing` 模块，跑一遍带标签的输出。

use rt_48_uuid::{Entity, new_id, new_ids, nil_id, parse_id, to_hyphenated, to_simple};

fn main() {
    println!("== 生成 v4 随机 UUID ==");
    let id = new_id();
    println!("新 ID: {id}");
    println!("版本号: {:?}", id.get_version_num());

    println!("\n== 批量生成 ==");
    for (i, id) in new_ids(3).into_iter().enumerate() {
        println!("  [{i}] {id}");
    }

    println!("\n== 给实体分配 ID ==");
    let alice = Entity::new("alice");
    let bob = Entity::new("bob");
    println!("{} -> {}", alice.name, alice.id);
    println!("{} -> {}", bob.name, bob.id);

    println!("\n== 解析与格式化 ==");
    let sample = "550e8400-e29b-41d4-a716-446655440000";
    match parse_id(sample) {
        Ok(parsed) => {
            println!("解析成功: {parsed}");
            println!("带连字符: {}", to_hyphenated(&parsed));
            println!("无连字符: {}", to_simple(&parsed));
        }
        Err(e) => println!("解析失败: {e}"),
    }

    println!("\n== 解析失败示例 ==");
    match parse_id("not-a-uuid") {
        Ok(_) => println!("不应到这里"),
        Err(e) => println!("如预期解析失败: {e}"),
    }

    println!("\n== nil UUID ==");
    println!("nil: {}", nil_id());
}
