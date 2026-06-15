# 发布检查清单

## 网站

- `cd website && pnpm install`
- `cd website && pnpm build`
- 桌面端章节页可读。
- 移动端章节页可读。
- 深色模式可切换。
- Pagefind 搜索索引已生成。

## 示例

- `./scripts/check-examples.sh`
- 所有示例 README 包含运行命令和预期输出。
- 新增章节的 `exampleDir` 能对应到实际目录。

## 内容

- 新章节有完整 frontmatter。
- 新章节出现在学习路线页。
- 标签页能显示新章节。
- 章节页上一篇/下一篇正确。
