# 03 Scalar And Compound Types

对应章节：`标量类型与复合类型`

## 运行

```bash
cd examples
cargo run -p rt_03_scalar_compound_types
cargo test -p rt_03_scalar_compound_types
```

## 预期输出

```text
30°C = 86°F, hot: true
first = 10, last = 40
```

## 关键知识点

- `u32`、`f64`、`bool`、`char` 是常见标量类型。
- tuple 可以组合不同类型。
- array 长度固定，长度也是类型的一部分。
