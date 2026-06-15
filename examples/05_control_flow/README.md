# 05 Control Flow

对应章节：`控制流`

## 运行

```bash
cd examples
cargo run -p rt_05_control_flow
cargo test -p rt_05_control_flow
```

## 预期输出

```text
score 92 => A
sum 1..=5 => 15
countdown: [3, 2, 1]
loop result: 4
```

## 关键知识点

- `if` 可以作为表达式。
- `for` 适合遍历 range。
- `while` 适合条件循环。
- `loop` 可以用 `break value` 返回值。
