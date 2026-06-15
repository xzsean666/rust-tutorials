import fs from "node:fs";
import path from "node:path";

/**
 * examples/ 仓库根目录。构建/开发都在 website/ 目录下运行，
 * 故以 cwd 为锚定位同级的 ../examples（比 import.meta.url 更稳：
 * 打包后产物路径会变，import.meta.url 不再指向源码位置）。
 */
const examplesRoot = path.resolve(process.cwd(), "../examples");

export interface ExampleFile {
  /** 仓库相对路径，如 examples/06_ownership/src/main.rs */
  path: string;
  /** Shiki 语言标识 */
  lang: string;
  /** 文件完整内容 */
  code: string;
  /** 是否默认展开（主文件 main.rs / lib.rs） */
  primary: boolean;
}

const LANG_BY_EXT: Record<string, string> = {
  ".rs": "rust",
  ".toml": "toml",
};

/** 递归收集目录下的源文件，跳过 target/ 等构建产物。 */
function collectSourceFiles(dir: string): string[] {
  const out: string[] = [];
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    if (entry.name === "target" || entry.name.startsWith(".")) continue;
    const full = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      out.push(...collectSourceFiles(full));
    } else if (entry.isFile()) {
      out.push(full);
    }
  }
  return out;
}

/** 排序权重：main.rs、lib.rs 最前，其余 src 文件居中，Cargo.toml 最后。 */
function sortWeight(relPath: string): number {
  if (relPath.endsWith("src/main.rs")) return 0;
  if (relPath.endsWith("src/lib.rs")) return 1;
  if (relPath.endsWith("Cargo.toml")) return 9;
  return 5;
}

/**
 * 读取某个示例目录的全部源文件，供页面完整展示。
 * 仅纳入 Cargo.toml 与 src/ 下的源码；排除 README、target 等。
 */
export function getExampleFiles(dir: string): ExampleFile[] {
  const root = path.join(examplesRoot, dir);
  if (!fs.existsSync(root)) {
    throw new Error(`示例目录不存在: examples/${dir}`);
  }

  const files = collectSourceFiles(root).filter((full) => {
    const ext = path.extname(full);
    const base = path.basename(full);
    return ext === ".rs" || base === "Cargo.toml";
  });

  return files
    .map((full) => {
      const relFromExamples = path.relative(examplesRoot, full).split(path.sep).join("/");
      const repoPath = `examples/${relFromExamples}`;
      const base = path.basename(full);
      return {
        path: repoPath,
        lang: LANG_BY_EXT[path.extname(full)] ?? "text",
        code: fs.readFileSync(full, "utf8").replace(/\s+$/, "\n"),
        primary: base === "main.rs" || base === "lib.rs",
      } satisfies ExampleFile;
    })
    .sort((a, b) => sortWeight(a.path) - sortWeight(b.path) || a.path.localeCompare(b.path));
}
