# 内容写作规范

每章教程放在 `website/src/content/tutorials/`，使用 MDX。

## Frontmatter

必填字段：

- `title`：章节标题。
- `description`：一句话说明本章目标。
- `stage`：学习阶段。
- `order`：全站排序号。
- `level`：`beginner`、`intermediate` 或 `advanced`。
- `estimatedMinutes`：预计学习分钟数。
- `tags`：导航标签。
- `rustConcepts`：概念索引。
- `prerequisites`：前置章节 id。
- `draft`：是否草稿。
- `updatedAt`：更新时间，格式 `YYYY-MM-DD`。

如果章节有配套示例，增加：

```yaml
exampleDir: "00_hello_world"
```

## 章节结构

推荐结构：

```mdx
# 章节标题

## 你会学到什么

## 最小示例

## 运行代码

## 代码讲解

## 常见错误

## 练习

## 小结

## 下一步
```

每章只讲一个主要主题。代码片段应优先来自 `examples/` 下的可运行项目。
