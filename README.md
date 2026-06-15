# Rust Tutorial

面向中文开发者的 Rust 学习平台。当前仓库先落地 V1 的基础骨架：

- `docs/`：产品与内容维护文档。
- `examples/`：可编译、可测试的 Rust 示例 workspace。
- `website/`：Astro + MDX + Pagefind 的静态教程网站。
- `scripts/`：示例校验脚本。

## 本地开发

Node.js 部分统一使用 `pnpm`：

```bash
cd website
pnpm install
pnpm dev
```

Rust 示例校验：

```bash
./scripts/check-examples.sh
```

网站生产构建：

```bash
cd website
pnpm build
```
