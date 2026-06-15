# 02 Variables

对应章节：`变量、可变性与 shadowing`

## 运行

```bash
cd examples
cargo run -p rt_02_variables
cargo test -p rt_02_variables
```

## 预期输出

```text
当前阶段: 2/16
已完成示例: 1
```

## 关键知识点

- 变量默认不可变。
- `mut` 表示同一个绑定可以被重新赋值。
- shadowing 会创建新绑定。
- `const` 必须显式标注类型。
