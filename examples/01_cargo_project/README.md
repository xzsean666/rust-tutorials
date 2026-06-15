# 01 Cargo Project

对应章节：`Cargo 基础`

## 运行

```bash
cd examples
cargo run -p rt_01_cargo_project
cargo test -p rt_01_cargo_project
```

## 预期输出

```text
package rt_01_cargo_project uses Rust 2024 Edition
cargo check / build / run / test use the same package metadata.
```

## 关键知识点

- `Cargo.toml` 描述 package。
- `cargo run -p` 可以运行 workspace 中的指定 package。
- 单元测试可以和二进制示例放在同一个文件中。
