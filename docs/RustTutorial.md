# Rust Tutorial 中文学习平台开发文档 V4

> 技术基线（2026-06 更新）：Rust 1.85+ / 2024 Edition，Astro 6（Content Layer API + Server Islands），搜索使用 Pagefind。

## 1. 项目定位

Rust Tutorial 是一个面向中文开发者的 Rust 学习平台。

它不是博客、不是 API 文档、也不是语法速查表；它的目标是把 Rust 官方学习资料、中文解释、可运行代码和真实项目结构组合成一条可持续维护的学习路线。

核心目标：

- 提供从零基础到项目实战的系统化 Rust 学习路线。
- 每个知识点都有可运行、可验证的代码示例。
- 用中文解释 Rust 的关键心智模型，包括所有权、借用、生命周期、错误处理、模块系统、异步和工程组织。
- 支持 PC 和手机阅读，支持顺序学习和自由查阅。
- 新增教程时尽量只增加内容文件和示例代码，不修改前端业务代码。
- 长期保持示例能编译、章节能搜索、学习进度能恢复。

非目标：

- 不复制 Rust 官方文档全文。
- 不做完整 crate API 文档站。
- 不在 V1 做用户系统、评论系统或服务端运行环境。
- 不为了炫技引入复杂后端；内容网站优先静态化。

## 2. 官方资料与版本基线

内容以官方资料为权威来源，中文教程负责解释、串联和补充可运行练习。

资料基线：

- Rust 语言主线：The Rust Programming Language。
- 示例风格：Rust By Example。
- 工具链与包管理：Cargo Book、rustup、rustdoc。
- API 查阅：Rust Standard Library 文档。
- Web 技术栈：Astro 官方文档、MDX、Content Layer API、Shiki。

Rust 版本策略：

- 教程默认使用 stable Rust，最低基线为 Rust 1.85（2024 Edition 随该版本稳定）。
- 示例项目默认使用 Rust 2024 Edition，除非章节专门讲历史版本差异。
- 升级旧示例到新 Edition 时使用 `cargo fix --edition`，并说明改动点。
- 仓库实现时应增加 `rust-toolchain.toml` 固定最低验证版本。
- 所有示例必须能通过 `cargo fmt`、`cargo clippy` 和 `cargo test`。
- 文档中如出现依赖版本，应说明“以仓库锁定版本为准”，避免教程和代码漂移。
- 优先使用语言原生能力（如 trait 中的 `async fn`），仅在确有需要时再引入外部 crate（如需要 `dyn` 时才用 `async-trait`）。

## 3. 目标用户

主要用户：

- 零基础 Rust 学习者。
- 有 Go、Java、Python、JavaScript、C/C++ 背景，想迁移到 Rust 的开发者。
- 已经会基础语法，但需要系统理解所有权、生命周期、模块系统和异步模型的开发者。
- 想通过小项目学习 Rust 工程实践的开发者。

用户需要解决的问题：

- 不知道学习顺序。
- 看懂了语法，但写不出可运行项目。
- 被 borrow checker、生命周期、trait bound、错误处理卡住。
- 不知道 crate、workspace、lib/bin、模块路径应该怎样组织。
- 学会语法后不知道如何使用 serde、tokio、reqwest、tracing、clap 等常见生态库。

## 4. 设计原则

内容优先：

- 技术栈只服务于阅读、搜索、进度和示例展示。
- 页面设计保持克制，避免把教程网站做成营销页。

示例优先：

- 每章至少一个可运行示例。
- 示例代码不能只是片段；关键章节要提供完整 `Cargo.toml`、目录结构、运行命令和输出结果。
- 文档里的代码片段应尽量来自 `examples/`，减少复制后失效。

中文优先：

- 正文用中文解释。
- Rust 关键字、类型名、crate 名、命令和错误码保留英文。
- 第一次出现核心术语时给出中英对照，例如“所有权 ownership”“借用 borrowing”。

移动端优先：

- 手机阅读时目录、进度、搜索和代码块都必须可用。
- 代码块允许横向滚动，不强行折行破坏格式。

长期维护：

- 章节元数据结构化。
- 教程目录、路由、标签、上一篇/下一篇、搜索索引自动生成。
- 示例代码进入 CI，避免教程长期腐化。

渐进学习：

- 每章只引入少量新概念。
- 先给可运行结果，再解释背后的机制。
- 所有权、生命周期、trait bound 等难点要拆成多章，而不是一次性讲完。

## 5. 技术选型

### 5.1 网站框架

使用 Astro（基线 Astro 6，2026 年 3 月稳定）。

原因：

- 适合内容型网站。
- 默认静态生成，性能好。
- 支持 Markdown、MDX，以及 Astro 5+ 引入的 Content Layer API。
- 可以按需引入交互组件，避免整站变成重客户端应用。
- Server Islands 支持把少量动态内容延迟到运行时，而站点主体仍是静态。
- 适合部署到 Cloudflare、GitHub Pages、Netlify、Vercel 等静态平台。

注意：Astro 6 要求 Node `^22.12.0 || ^24.0.0`，仓库需在 `package.json` 与 CI 中固定 Node 版本。

### 5.2 开发语言

使用 TypeScript。

用途：

- 内容元数据类型校验。
- 搜索索引生成。
- 进度状态和少量前端交互。
- 构建脚本和站点配置。

### 5.3 样式方案

使用 Tailwind CSS。

要求：

- 建立少量全局设计 token：颜色、字号、间距、代码块、边框、阴影。
- 不做单一色系页面，深浅色都要保证对比度。
- 教程页以阅读效率为核心，避免大面积装饰卡片。

### 5.4 内容系统

使用 Astro Content Layer API（Content Collections 的新一代实现）+ MDX。

用途：

- 每章教程用 MDX 编写。
- 集合在 `src/content.config.ts` 中用 `glob()` loader 加载本地 MDX，元数据由 Zod schema 校验。
- 相比旧版 `type: "content"` 写法，Content Layer 构建更快、内存占用更低，且日后可平滑接入远程数据源。
- 自动生成学习路线、标签页、搜索索引和上一篇/下一篇。
- 少量教学组件可在 MDX 中使用，例如 `FileTree`、`RunCommand`、`OutputBlock`、`Callout`、`Quiz`。

说明：旧的 `type: "content"` 集合定义在 Astro 6 已结束兼容宽限期，新仓库直接使用 loader 写法，不要再用 legacy 模式。

### 5.5 代码高亮

使用 Astro 内置 Shiki 能力。

支持语言：

- Rust
- TOML
- Bash
- JSON
- YAML
- TypeScript
- Markdown

代码块要求：

- 支持复制。
- 支持文件名展示。
- 支持多文件示例切换。
- 代码块在移动端稳定横向滚动。
- 使用 Shiki 双主题（light/dark），跟随站点主题切换，不在客户端做高亮重算。
- 如需文件名标签、行高亮、diff 标记等增强能力，可评估 Expressive Code（Starlight 同款），但保持配置克制。

### 5.6 搜索

V1 使用 Pagefind（构建后扫描产物 HTML，生成分片二进制索引）。

选择理由：

- 按需加载索引分片，初始包小，内容增长后仍然稳定，无需更换方案。
- 零配置抓取已渲染 HTML，不需要手工维护索引脚本。
- 原生支持中文等多语言分词，并提供可直接复用的 UI 组件。
- 是 Astro 官方文档框架 Starlight 的默认搜索方案，生态成熟。

需要参与排序/过滤的元数据（标签、阶段、难度、Rust 概念、示例目录）通过 Pagefind 的 `data-pagefind-filter` 与 `data-pagefind-meta` 标注在页面上，由 Pagefind 索引。

备选：内容很少（不足约 100 章）或需要高度定制模糊匹配时，可改用静态索引 + Fuse.js；两种方案都不需要服务端。

### 5.7 状态与存储

V1 不需要登录系统。

本地状态使用 `localStorage`：

- 已完成章节。
- 最近阅读章节。
- 主题偏好。
- 字号偏好。
- 搜索历史，可选。

如果跨组件状态明显增多，再引入 Nano Stores；V1 不应为简单进度状态提前增加复杂度。

### 5.8 部署

默认静态站部署。

推荐策略：

- 新项目优先评估 Cloudflare Workers 静态资源部署。
- 如果只需要静态页面，也可以部署到 Cloudflare Pages。
- 构建产物不依赖后端数据库。
- 所有搜索和学习进度都在客户端完成。

## 6. 仓库结构

建议结构：

```text
rust-tutorials/
  docs/
    RustTutorial.md
    CONTENT_GUIDE.md
    RELEASE_CHECKLIST.md
  examples/
    00_hello_world/            # 阶段 0
    01_cargo_project/
    02_variables/              # 阶段 1
    03_scalar_compound_types/
    04_functions/
    05_control_flow/
    06_ownership_move/         # 阶段 2
    07_borrowing/
    08_slices/
    09_lifetimes/
    10_struct/                 # 阶段 3
    11_enum/
    12_match_patterns/
    13_strings/                # 阶段 4
    14_collections/
    15_option_result/
    16_error_handling/
    17_file_io/
    20_generics/               # 阶段 5
    21_traits/
    22_trait_objects/
    23_closures/
    24_iterators/              # 阶段 6
    25_custom_iterator/
    26_itertools/
    27_box_deref/              # 阶段 7
    28_rc_refcell/
    29_weak_cow/
    30_threads/                # 阶段 8
    31_shared_state/
    32_channels/
    33_rayon/
    40_module_basic/           # 阶段 9
    41_workspace/
    42_features/
    50_unit_test/              # 阶段 10
    51_integration_test/
    52_doc_test/
    53_criterion_bench/
    60_async_basic/            # 阶段 11
    61_tokio_tasks/
    62_async_channels/
    63_streams/
    70_serde_json/             # 阶段 12
    71_serde_formats/
    72_thiserror_anyhow/
    73_clap_cli/
    74_tracing_log/
    75_reqwest_http/
    76_datetime/
    77_regex/
    78_config_loader/
    79_rand_uuid/
    80_axum_hello/             # 阶段 13
    81_axum_json_state/
    82_sqlx_crud/
    90_declarative_macro/      # 阶段 14
    91_proc_macro_intro/
    92_unsafe_ffi/
    a0_cli_todo/               # 阶段 15（真实项目）
    a1_config_loader/
    a2_http_client/
    a3_task_queue/
    a4_axum_shortener/
    a5_file_batch/
  website/
    astro.config.mjs
    package.json
    src/
      content.config.ts
      content/
        tutorials/
      components/
      layouts/
      pages/
      styles/
      utils/
    public/
  scripts/
    check-examples.sh
  README.md
```

目录约定：

- `docs/` 放项目开发文档和维护规范。
- `examples/` 放所有 Rust 示例工程。
- `website/` 放 Astro 站点。
- `scripts/` 放示例校验和发布辅助脚本（搜索索引由 Pagefind 在构建后自动生成，无需手写索引脚本）。
- `src/content.config.ts` 为 Astro 6 约定路径（旧版 `src/content/config.ts` 已弃用）。

## 7. 内容模型

每章教程必须有结构化元数据，集合在 `src/content.config.ts` 中用 Content Layer 的 `glob()` loader 定义：

```ts
import { defineCollection, z } from "astro:content";
import { glob } from "astro/loaders";

const tutorials = defineCollection({
  loader: glob({ pattern: "**/*.mdx", base: "./src/content/tutorials" }),
  schema: z.object({
    // 见下方字段
  }),
});

export const collections = { tutorials };
```

建议 schema（用 Zod 表达，构建时强校验）：

```ts
{
  title: string;
  description: string;
  stage:
    | "intro"
    | "basic"
    | "ownership"
    | "modeling"
    | "std"
    | "generics"
    | "iterators"
    | "smart-pointers"
    | "concurrency"
    | "crate"
    | "quality"
    | "async"
    | "ecosystem"
    | "web-db"
    | "advanced"
    | "project";
  order: number;
  level: "beginner" | "intermediate" | "advanced";
  estimatedMinutes: number;
  tags: string[];
  rustConcepts: string[];
  prerequisites: string[];
  exampleDir?: string;
  draft: boolean;
  updatedAt: string;
}
```

MDX frontmatter 示例：

```mdx
---
title: "变量与可变性"
description: "理解 let、mut、shadowing 和 const 的区别。"
stage: "basic"
order: 20
level: "beginner"
estimatedMinutes: 25
tags: ["基础语法", "变量"]
rustConcepts: ["let", "mut", "shadowing", "const"]
prerequisites: ["hello-world"]
exampleDir: "02_variables"
draft: false
updatedAt: "2026-06-15"
---
```

生成能力：

- `order` 生成上一篇/下一篇。
- `stage` 生成学习路线。
- `tags` 生成标签页。
- `rustConcepts` 生成概念索引。
- `exampleDir` 生成 GitHub 示例链接。

## 8. 课程路线

课程按从零到项目实战的顺序分为 16 个阶段。每个“章节”对应一篇 MDX 教程，原则上一篇只讲一个主题；标注 `★` 的为重点难点，建议拆分讲解或配套更多练习。示例目录用数字前缀保证顺序，命名见第 10 节。

学习路线总览：

| 阶段 | 主题 | 难度 | 对应 stage |
|------|------|------|-----------|
| 0 | 环境与学习方式 | beginner | `intro` |
| 1 | 基础语法 | beginner | `basic` |
| 2 | 所有权、借用、生命周期 | beginner→intermediate | `ownership` |
| 3 | 复合类型与模式匹配 | beginner | `modeling` |
| 4 | 标准库与错误处理 | beginner→intermediate | `std` |
| 5 | 泛型、trait 与闭包 | intermediate | `generics` |
| 6 | 迭代器与函数式 | intermediate | `iterators` |
| 7 | 智能指针与内部可变性 | intermediate | `smart-pointers` |
| 8 | 并发与多线程 | intermediate→advanced | `concurrency` |
| 9 | 模块系统、crate 与 workspace | intermediate | `crate` |
| 10 | 测试、文档与工程质量 | intermediate | `quality` |
| 11 | 异步编程 | intermediate→advanced | `async` |
| 12 | 常用生态库 | intermediate | `ecosystem` |
| 13 | Web 与数据库 | advanced | `web-db` |
| 14 | 宏、unsafe 与 FFI | advanced | `advanced` |
| 15 | 真实项目 | advanced | `project` |

### 阶段 0：环境与学习方式（intro）

目标：让用户能在本地运行 Rust，并理解教程的学习方式。

章节：

- Rust 是什么，适合与不适合解决的问题。
- 安装 rustup，理解工具链（rustc、cargo、clippy、rustfmt）。
- 第一个程序：Hello World。
- cargo 基础命令：`new`、`build`、`run`、`check`。
- 读懂 `Cargo.toml` 与 `Cargo.lock`。
- ★ 如何阅读编译器错误（`rustc --explain`、错误码）。
- 配置编辑器与 rust-analyzer。
- 如何运行本教程示例。

示例：

- `00_hello_world`
- `01_cargo_project`

### 阶段 1：基础语法（basic）

目标：写出简单可运行程序。

章节：

- 变量、可变性 `mut`、shadowing。
- 常量 `const` 与 `static`。
- 标量类型：整数、浮点、`bool`、`char`，以及溢出行为。
- 复合类型：tuple、array。
- 函数、参数与返回值。
- 语句与表达式（block 表达式、隐式返回）。
- 注释与文档注释。
- 控制流：`if` / `else` / `if let`。
- 循环：`loop`、`while`、`for`，循环标签与 `loop` 返回值。
- 用 `println!` / `format!` 做基本格式化。

示例：

- `02_variables`
- `03_scalar_compound_types`
- `04_functions`
- `05_control_flow`

### 阶段 2：所有权、借用和生命周期（ownership）

目标：建立 Rust 最核心的内存模型。这是初学者最大的难点，单独拆细。

章节：

- 栈与堆，内存模型直觉。
- ★ ownership：移动语义 move。
- `Copy` 与 `Clone` 的区别。
- borrowing：不可变借用 `&`。
- ★ 可变借用 `&mut` 与借用规则（同一时刻只能有一个可变借用）。
- 引用作用域与 NLL（non-lexical lifetimes）。
- slice：`&str` 与 `&[T]`。
- ★ 常见 borrow checker 错误与修复套路。
- 函数中的所有权与引用传递。
- lifetime 的直觉：为什么需要、它在表达什么。
- lifetime 标注语法 `'a` 与函数签名。
- struct 中持有引用。
- lifetime elision（省略）规则。
- 何时应该 `clone`：成本权衡。

示例：

- `06_ownership_move`
- `07_borrowing`
- `08_slices`
- `09_lifetimes`

### 阶段 3：复合类型与模式匹配（modeling）

目标：掌握 Rust 的数据建模方式。

章节：

- struct：定义、实例化、字段访问。
- struct update 语法、tuple struct、unit struct。
- `impl`：method 与 associated function。
- enum：定义与携带数据。
- `Option` 初识。
- ★ `match` 表达式与穷尽性检查。
- 模式：绑定、解构、通配 `_`、范围、`|`、守卫 `if`。
- `if let` / `let else` / `while let`。
- 派生宏 `derive`（`Debug`、`Clone`、`PartialEq` 等）。

示例：

- `10_struct`
- `11_enum`
- `12_match_patterns`

### 阶段 4：标准库与错误处理（std）

目标：能写出日常程序。

章节：

- `String` 与 `&str` 深入（拼接、切片、UTF-8 注意点）。
- `Vec` 常用操作。
- `HashMap`、`BTreeMap`、`HashSet`。
- `Option` 的方法链（`map`、`and_then`、`unwrap_or` 等）。
- `Result` 与可恢复错误。
- `panic!` 与不可恢复错误的边界。
- ★ 错误传播 `?` 运算符。
- 自定义错误类型与 `From` 转换。
- `Box<dyn Error>` 作为通用错误。
- 文件读写 `std::fs`。
- 标准输入输出、命令行参数 `std::env::args`。
- 格式化：`Display` vs `Debug`，`format!` 进阶。

示例：

- `13_strings`
- `14_collections`
- `15_option_result`
- `16_error_handling`
- `17_file_io`

### 阶段 5：泛型、trait 与闭包（generics）

目标：掌握 Rust 的抽象与复用机制，是进入“中级”的关键阶段。

章节：

- 泛型函数。
- 泛型 struct 与 enum。
- 泛型方法与约束。
- ★ trait：定义与实现。
- 默认方法、supertrait（trait 继承）。
- ★ trait bound：`where` 子句、`impl Trait` 参数。
- 返回 `impl Trait`。
- 运算符重载（`Add`、`PartialOrd` 等）。
- 关联类型 associated type。
- ★ 静态分发 vs 动态分发（`dyn Trait`、trait object）。
- 对象安全 object safety。
- 标准库常见 trait：`From`/`Into`、`TryFrom`、`Default`、`Display`、`AsRef`。
- ★ 闭包：`Fn` / `FnMut` / `FnOnce`。
- `move` 闭包。
- 把闭包作为参数和返回值。

示例：

- `20_generics`
- `21_traits`
- `22_trait_objects`
- `23_closures`

### 阶段 6：迭代器与函数式（iterators）

目标：写出地道、零成本的迭代代码。

章节：

- `Iterator` trait 与 `next`。
- 惰性求值的含义。
- 适配器：`map` / `filter` / `take` / `skip` / `enumerate` / `zip`。
- 消费器：`collect` / `sum` / `fold` / `reduce` / `count`。
- 查找：`find` / `any` / `all` / `position`。
- `flat_map` / `flatten` / `chain`。
- 三种迭代方式：`iter` / `iter_mut` / `into_iter` 与 `IntoIterator`。
- 自定义迭代器（实现 `Iterator`）。
- 性能：迭代器 vs 手写循环（零成本抽象）。
- `itertools` 常用能力（`chunk_by`、`unique`、`itertools::izip!` 等）。

示例：

- `24_iterators`
- `25_custom_iterator`
- `26_itertools`

### 阶段 7：智能指针与内部可变性（smart-pointers）

目标：理解堆分配、共享所有权与内部可变性。

章节：

- `Box<T>`：堆分配与递归类型。
- `Deref` 与 deref coercion。
- `Drop` trait 与析构顺序。
- `Rc<T>`：共享所有权。
- ★ `RefCell<T>` 与内部可变性（运行期借用检查）。
- `Rc<RefCell<T>>` 组合模式。
- `Weak<T>` 与打破循环引用。
- `Cow<T>`：写时克隆。
- 选型指南：何时用哪种指针。

示例：

- `27_box_deref`
- `28_rc_refcell`
- `29_weak_cow`

### 阶段 8：并发与多线程（concurrency）

目标：掌握同步多线程编程与共享状态。

章节：

- `thread::spawn` 与 `join`。
- `move` 闭包与线程间数据转移。
- ★ `Send` 与 `Sync`。
- 共享状态：`Arc<Mutex<T>>`。
- `RwLock` 与读多写少场景。
- 消息传递：`std::sync::mpsc` channel。
- 原子类型 `atomic` 与内存序简介。
- scoped threads（`std::thread::scope`）。
- 数据并行：`rayon` 入门（`par_iter`）。
- 常见并发陷阱：死锁、`Mutex` 中毒、伪共享。

示例：

- `30_threads`
- `31_shared_state`
- `32_channels`
- `33_rayon`

### 阶段 9：模块系统、crate 与 workspace（crate）

目标：理解 Rust 工程组织。

章节：

- `mod` 与文件/目录组织。
- 路径：`crate` / `self` / `super`，绝对与相对路径。
- 可见性：`pub`、`pub(crate)`、`pub(super)`。
- `use` 与重导出 `pub use`。
- lib crate 与 bin crate。
- package 与 crate 的区别、多 bin 项目。
- workspace 组织多 crate。
- 依赖管理：添加、版本、特性选择。
- ★ feature flag 设计与条件编译 `cfg`。
- Cargo profile（dev/release）与编译优化。
- Edition 是什么，2024 Edition 关键变化，`cargo fix --edition` 迁移。

示例：

- `40_module_basic`
- `41_workspace`
- `42_features`

### 阶段 10：测试、文档与工程质量（quality）

目标：把代码写得可维护、可验证。

章节：

- 单元测试 `#[test]` 与断言宏（`assert!`、`assert_eq!`）。
- 测试组织：`#[cfg(test)] mod tests`。
- 集成测试：`tests/` 目录。
- 测试 `Result` 与 `should_panic`。
- 文档注释与文档测试（doc test）。
- `cargo doc` 生成 API 文档。
- 基准测试：`criterion`。
- 属性测试：`proptest`。
- mock 与依赖注入：`mockall`。
- `clippy` 与 `rustfmt` 实践。
- 简单 CI 流程（fmt + clippy + test）。

示例：

- `50_unit_test`
- `51_integration_test`
- `52_doc_test`
- `53_criterion_bench`

### 阶段 11：异步编程（async）

目标：建立 async 心智模型并能用 tokio 写并发程序。

章节：

- 同步 vs 异步：要解决什么问题。
- ★ async/await 心智模型。
- `Future` trait 与 `poll` 简介。
- 运行时：tokio 入门、`#[tokio::main]` 与 runtime 配置。
- tokio task 与 `spawn`。
- 并发组合：`join!`、`select!`。
- trait 中的 `async fn`（2024 Edition 原生支持，静态分发）。
- async 闭包 `async || {}`（Rust 1.85 稳定）。
- 何时仍需 `async-trait`（需要 `dyn` 动态分发时）。
- async channel：`tokio::sync::mpsc`。
- 异步共享状态：`tokio::sync::Mutex`。
- 取消与超时（`tokio::time::timeout`）。
- `Stream` 与 `tokio-stream`。
- ★ 常见错误：在异步上下文里阻塞、`Send` 边界、`.await` 持锁。

示例：

- `60_async_basic`
- `61_tokio_tasks`
- `62_async_channels`
- `63_streams`

### 阶段 12：常用生态库（ecosystem）

目标：学会组合工程中最常用的 crate。按用途分组，每组一到多篇。

序列化与配置：

- serde + serde_json：派生 `Serialize`/`Deserialize`。
- 多格式：`toml`、`serde_yaml`，以及 `#[serde(...)]` 常用属性。
- 配置加载：`config` / `figment` + 环境变量分层。

错误处理：

- thiserror（库层结构化错误，`thiserror` 2）。
- anyhow（应用层错误聚合与上下文）。

命令行：

- clap（`clap` 4，derive 风格、子命令、参数校验）。
- 进度条 `indicatif`、交互提示 `dialoguer`。

日志与可观测：

- tracing + tracing-subscriber：分级、结构化字段、span。

网络：

- reqwest（`reqwest` 0.13，默认 rustls）：GET/POST、JSON、超时、重试。

时间、随机与 ID：

- 日期时间：`chrono` 或 `time`。
- 随机数 `rand`、唯一 ID `uuid`。

文本与工具：

- 正则 `regex`。
- 迭代增强 `itertools`。
- 惰性初始化：`std::sync::LazyLock` / `once_cell`。
- 并发哈希表 `dashmap`。

示例：

- `70_serde_json`
- `71_serde_formats`
- `72_thiserror_anyhow`
- `73_clap_cli`
- `74_tracing_log`
- `75_reqwest_http`
- `76_datetime`
- `77_regex`
- `78_config_loader`
- `79_rand_uuid`

### 阶段 13：Web 与数据库（web-db）

目标：理解服务端开发常见组件，能写出小型 API 服务。

章节：

- axum 入门（`axum` 0.8）：路由、handler、提取器 extractor。
- 共享状态 `State` 与依赖注入。
- JSON 请求与响应。
- 中间件与 `tower` 生态（日志、超时、限流）。
- 统一错误处理与 `IntoResponse`。
- 数据库：`sqlx`（编译期校验 SQL）与连接池。
- 数据库迁移 migrations。
- ORM 选型简介：`sea-orm`。
- 组合一个 REST API（CRUD）。

示例：

- `80_axum_hello`
- `81_axum_json_state`
- `82_sqlx_crud`

V1 可先覆盖 axum 基础；数据库（sqlx）可作为 V1.5 或 V2。

### 阶段 14：宏、unsafe 与 FFI（advanced）

目标：理解元编程与底层能力，知道何时该用、何时该避免。

章节：

- 声明宏 `macro_rules!` 基础与常见模式。
- 过程宏简介：derive / attribute / function-like（概念 + 最小示例）。
- 何时该写宏、何时不该。
- unsafe Rust：裸指针、`union`、`static mut` 的风险。
- ★ 为 unsafe 提供安全封装的原则（不变量、文档化 SAFETY）。
- FFI：调用 C 函数、把 Rust 暴露给 C。
- 性能优化与 profiling 简介（`cargo flamegraph`、`perf`）。

示例：

- `90_declarative_macro`
- `91_proc_macro_intro`
- `92_unsafe_ffi`

### 阶段 15：真实项目（project）

目标：把前面知识点组合成完整项目。每个项目标注用到的核心 crate 与对应阶段。

项目清单：

- CLI 待办工具：clap + serde + anyhow（阶段 4/12）。
- JSON/TOML 配置加载器：serde + config（阶段 12）。
- HTTP API 客户端（如 GitHub/天气）：reqwest + tokio + serde（阶段 11/12）。
- 结构化日志组件：tracing + tracing-subscriber（阶段 12）。
- 并发任务队列：tokio + channel + Arc/Mutex（阶段 8/11）。
- URL 短链服务：axum + sqlx（阶段 13）。
- 文件批处理工具：rayon + walkdir（阶段 8）。
- 进阶：表达式计算器/小型解释器：enum + match + 自定义错误（阶段 3/4/5）。

每个项目必须包含：

- 需求说明。
- 目录结构。
- 核心设计。
- 完整代码。
- 测试。
- 运行方式。
- 常见扩展方向。

示例：

- `a0_cli_todo`
- `a1_config_loader`
- `a2_http_client`
- `a3_task_queue`
- `a4_axum_shortener`
- `a5_file_batch`

## 9. 章节规范

每个章节必须包含：

- 标题和一句话目标。
- 学习目标。
- 前置知识。
- 最小可运行示例。
- 代码讲解。
- 常见错误。
- 练习题。
- 章节总结。
- 下一章导航。

推荐章节结构：

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

写作要求：

- 每章只解决一个清晰主题。
- 第一个代码示例必须能直接运行。
- 对编译错误要展示错误原因和修复方式。
- 不用“很简单”“显然”这类降低初学者体验的表达。
- 所有命令都写明运行目录。
- 输出结果应写成可对照的文本块。

## 10. 示例代码系统

每个示例目录都是独立 Cargo 项目，或 workspace 中的独立 member。

示例目录结构：

```text
examples/01_variables/
  Cargo.toml
  src/
    main.rs
  README.md
```

示例 README 必须包含：

- 对应教程章节。
- 运行命令。
- 预期输出。
- 关键知识点。

示例命名规则：

- 使用数字前缀保证顺序。
- 使用 snake_case。
- 一个示例只承载一个主要知识点。
- 项目实战示例可以包含多个模块，但必须有清晰 README。

质量要求：

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-targets`
- 需要网络的示例必须提供 mock 或离线模式。
- 异步和网络示例要避免依赖不稳定外部服务。

## 11. 页面结构

V1 页面：

- 首页。
- 学习路线页。
- 章节详情页。
- 搜索页。
- 标签页。
- 示例索引页。
- 关于页。

首页展示：

- 项目定位。
- 推荐学习路径。
- 当前章节总数。
- 最近更新章节。
- GitHub 仓库链接。

学习路线页展示：

- 分阶段路线。
- 每个阶段的章节列表。
- 每章预计学习时间。
- 完成状态。
- 推荐下一章。

章节页展示：

- 标题。
- 难度。
- 预计学习时间。
- 标签。
- 正文。
- 目录。
- 代码高亮。
- 示例目录链接。
- 上一篇和下一篇。
- 标记完成按钮。
- 编辑此页。
- 提交问题。

搜索页展示：

- 搜索框。
- 搜索结果。
- 标签过滤。
- 阶段过滤。
- 难度过滤。

## 12. 阅读体验

桌面端：

- 左侧章节导航。
- 中间正文。
- 右侧当前页目录。
- 顶部搜索入口。

移动端：

- 顶部标题栏。
- 折叠菜单。
- 章节目录抽屉。
- 代码块横向滚动。
- 底部上一篇/下一篇。

深色模式：

- 支持 light。
- 支持 dark。
- 支持跟随系统。
- 用户选择写入 `localStorage`。

代码展示：

- 显示语言。
- 显示文件名。
- 支持复制。
- 多文件示例使用 tabs。
- 终端输出和源码使用不同样式。

## 13. 学习进度

V1 使用本地进度。

状态字段：

```json
{
  "completed": ["hello-world", "variables"],
  "lastRead": "ownership-move",
  "updatedAt": "2026-06-15T00:00:00.000Z",
  "theme": "system",
  "fontScale": 1
}
```

功能：

- 标记章节已完成。
- 显示总进度。
- 继续上次阅读。
- 清空本地进度。

约束：

- 不需要登录。
- 不跨设备同步。
- 存储结构要带版本号，方便以后迁移。

## 14. 标签与概念索引

基础标签：

- 基础语法
- 所有权
- 借用
- 生命周期
- 复合类型
- 模式匹配
- 标准库
- 错误处理
- 泛型
- trait
- 闭包
- 迭代器
- 智能指针
- 内部可变性
- 并发
- 多线程
- 模块系统
- 工程组织
- 测试
- 异步
- 网络
- Web
- 数据库
- 宏
- unsafe
- FFI
- 工具链
- 生态库
- 实战项目

概念索引：

- `String` / `&str`
- `Vec` / `HashMap` / `BTreeMap` / `HashSet`
- `Option` / `Result` / `?`
- `Iterator` / `IntoIterator`
- `trait` / `dyn` / `impl Trait`
- `From` / `Into` / `TryFrom`
- `Fn` / `FnMut` / `FnOnce`
- `lifetime` / `'a`
- `Box` / `Rc` / `Arc` / `RefCell` / `Cow`
- `Mutex` / `RwLock` / `Send` / `Sync`
- `async` / `await` / `Future` / `Stream`
- `tokio` / `rayon`
- `serde` / `clap` / `tracing` / `reqwest` / `thiserror` / `anyhow` / `axum` / `sqlx`
- `macro_rules!` / 过程宏
- `unsafe` / FFI

标签用于导航，概念索引用于查漏补缺。

## 15. GitHub 集成

每个教程页面提供：

- 查看源码。
- 编辑文档。
- 提交问题。
- 查看对应示例。

实现方式：

- 根据内容文件路径生成 GitHub edit URL。
- 根据 `exampleDir` 生成示例链接。
- issue 模板中自动带入当前页面标题和 URL。

## 16. SEO 与可访问性

SEO：

- 生成 sitemap。
- 生成 robots.txt。
- 每页有 title、description、canonical。
- 生成 Open Graph 信息。
- 可选 RSS，展示最近更新章节。

可访问性：

- 页面语言设置为 `zh-CN`。
- 正文 heading 层级不能跳跃。
- 链接文本要有明确含义。
- 代码块复制按钮有可访问标签。
- 颜色对比度满足常规阅读要求。
- 键盘可以访问导航、搜索和主题切换。

## 17. V1 必须实现

V1 范围：

- Astro 站点基础结构。
- Content Collections。
- MDX 教程渲染。
- Shiki 代码高亮。
- 学习路线。
- 章节页。
- 搜索。
- 标签页。
- 示例索引。
- 手机适配。
- 深色模式。
- 本地学习进度。
- GitHub 链接。
- 静态部署。
- 示例代码校验脚本。

V1 不做：

- 用户账号。
- 评论。
- 在线编译运行。
- AI 助手。
- 复杂后台管理。
- 数据库。

## 18. V2 规划

候选能力：

- 在线运行 Rust 示例。
- 收藏。
- 跨设备同步。
- 评论或讨论。
- AI 问答助手。
- 知识图谱。
- 学习统计。
- 章节版本历史。
- Playground 集成增强。

V2 原则：

- 只有在 V1 内容和示例稳定后再引入。
- 任何服务端能力都必须有明确收益。
- 不牺牲静态阅读体验。

## 19. 实施里程碑

### Milestone 0：文档与骨架

交付：

- 项目 README。
- 本开发文档。
- 内容写作规范。
- 示例代码规范。
- Astro 项目骨架。

验收：

- `website` 能本地启动。
- 有 1 个示例章节和 1 个示例项目。

### Milestone 1：内容系统

交付：

- Content Collections schema。
- MDX 布局。
- 章节页。
- 上一篇/下一篇。
- 标签页。

验收：

- 新增 MDX 文件后自动出现在路线和标签中。
- 元数据错误会在构建时报错。

### Milestone 2：示例系统

交付：

- `examples/` 目录规范。
- 示例索引页。
- 校验脚本。
- CI 示例检查。

验收：

- 所有示例能格式化、lint、测试。
- 教程页能链接到对应示例。

### Milestone 3：阅读体验

交付：

- 首页。
- 学习路线页。
- 搜索页。
- 深色模式。
- 移动端导航。
- 本地学习进度。

验收：

- 手机和桌面都能完成顺序学习流程。
- 刷新页面后进度不丢失。

### Milestone 4：MVP 内容

交付：

- 阶段 0 到阶段 4 的全部核心章节（环境 → 基础 → 所有权 → 建模 → 标准库与错误处理）。
- 至少 20 个可运行示例。
- 每章有常见错误和练习。

验收：

- 初学者能从安装一路学到错误处理，写出日常小程序。
- 所有示例 CI 通过。

### Milestone 4.5：中级内容

交付：

- 阶段 5 到阶段 11 的核心章节（泛型/trait/闭包、迭代器、智能指针、并发、工程组织、测试、异步）。
- 中级阶段示例补齐。

验收：

- 学习者能理解抽象、并发与异步，组织多 crate 工程。

### Milestone 4.8：生态库与项目

交付：

- 阶段 12 到阶段 15 的章节（常用生态库、Web 与数据库、宏/unsafe/FFI、真实项目）。
- 至少 3 个完整真实项目。

验收：

- 学习者能用常用 crate 组合出可运行项目。

### Milestone 5：部署与发布

交付：

- 生产构建。
- sitemap。
- robots.txt。
- Open Graph。
- 部署流水线。

验收：

- 线上页面可访问。
- 搜索可用。
- 移动端可读。
- Lighthouse 基础指标达标。

## 20. 验收标准

站点构建：

```bash
cd website
pnpm install
pnpm build
```

示例校验：

```bash
cd examples
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

内容检查：

- 每章有 frontmatter。
- 每章有学习目标。
- 每章有运行命令。
- 每章有示例或明确说明为什么没有示例。
- 每章有小结和下一步。
- 链接无明显 404。

发布前检查：

- 桌面端章节页可读。
- 移动端章节页可读。
- 搜索可用。
- 深色模式可用。
- 学习进度可保存。
- GitHub 链接正确。

## 21. 风险与应对

内容漂移：

- 风险：Rust、crate 或官方文档更新后，教程过时。
- 应对：固定工具链版本，定期升级依赖，示例进入 CI。

示例失效：

- 风险：文档代码能看不能跑。
- 应对：示例必须是 Cargo 项目，CI 必须运行 fmt、clippy、test。

范围膨胀：

- 风险：过早加入用户系统、AI、在线运行，拖慢 MVP。
- 应对：V1 只做静态学习闭环。

移动端体验差：

- 风险：代码块、目录和导航在手机上难用。
- 应对：移动端优先设计，代码块横向滚动，导航折叠。

教程过难：

- 风险：一次讲太多概念导致初学者放弃。
- 应对：每章限制主题数量，常见错误单独解释，练习分基础和进阶。

## 22. 参考资料

官方资料：

- Rust Learn: <https://www.rust-lang.org/learn>
- The Rust Programming Language: <https://doc.rust-lang.org/book/>
- Rust By Example: <https://doc.rust-lang.org/rust-by-example/>
- Rust Standard Library: <https://doc.rust-lang.org/std/>
- Rust Edition Guide（含 2024 Edition）: <https://doc.rust-lang.org/edition-guide/>
- Cargo Book: <https://doc.rust-lang.org/cargo/>
- rustup Book: <https://rust-lang.github.io/rustup/>
- Astro Content Collections / Content Layer: <https://docs.astro.build/en/guides/content-collections/>
- Astro MDX: <https://docs.astro.build/en/guides/integrations-guide/mdx/>
- Astro Syntax Highlighting: <https://docs.astro.build/en/guides/syntax-highlighting/>
- Astro Cloudflare: <https://docs.astro.build/en/guides/deploy/cloudflare/>
- Pagefind: <https://pagefind.app/>

生态库资料：

- serde: <https://serde.rs/>
- tokio: <https://tokio.rs/>
- reqwest: <https://docs.rs/reqwest/>
- tracing: <https://docs.rs/tracing/>
- clap: <https://docs.rs/clap/>
- anyhow: <https://docs.rs/anyhow/>
- thiserror: <https://docs.rs/thiserror/>
- axum: <https://docs.rs/axum/>

依赖主版本基线（具体补丁版本以仓库 `Cargo.lock` 为准）：

- `serde` 1 / `serde_json` 1
- `tokio` 1.x
- `reqwest` 0.13
- `clap` 4
- `thiserror` 2 / `anyhow` 1
- `axum` 0.8

## 23. 最终目标

Rust Tutorial 的最终效果是一套中文、示例驱动、可长期维护的 Rust 交互式电子书。

它应该让用户完成三件事：

- 知道 Rust 应该按什么顺序学。
- 能运行每个知识点对应的代码。
- 能把基础语法组合成真实项目。
