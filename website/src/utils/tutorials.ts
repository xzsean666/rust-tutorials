import { getCollection, type CollectionEntry } from "astro:content";

export type Tutorial = CollectionEntry<"tutorials">;

export const stageLabels = {
  intro: "环境与学习方式",
  basic: "基础语法",
  ownership: "所有权、借用和生命周期",
  modeling: "复合类型与模式匹配",
  std: "标准库与错误处理",
  generics: "泛型、trait 与闭包",
  iterators: "迭代器与函数式",
  "smart-pointers": "智能指针与内部可变性",
  concurrency: "并发与多线程",
  crate: "模块系统、crate 与 workspace",
  quality: "测试、文档与工程质量",
  async: "异步编程",
  ecosystem: "常用生态库",
  "web-db": "Web 与数据库",
  advanced: "宏、unsafe 与 FFI",
  project: "真实项目",
} as const;

export const stageOrder = Object.keys(stageLabels) as Array<keyof typeof stageLabels>;

export async function getPublishedTutorials() {
  const tutorials = await getCollection("tutorials", ({ data }) => !data.draft);
  return tutorials.sort((left, right) => left.data.order - right.data.order);
}

export function tutorialPath(tutorial: Tutorial) {
  return `/tutorials/${tutorial.id}/`;
}

export function getTutorialNeighbors(tutorials: Tutorial[], currentId: string) {
  const index = tutorials.findIndex((tutorial) => tutorial.id === currentId);
  return {
    previous: index > 0 ? tutorials[index - 1] : undefined,
    next: index >= 0 && index < tutorials.length - 1 ? tutorials[index + 1] : undefined,
  };
}

export function groupTutorialsByStage(tutorials: Tutorial[]) {
  return stageOrder
    .map((stage) => ({
      stage,
      label: stageLabels[stage],
      tutorials: tutorials.filter((tutorial) => tutorial.data.stage === stage),
    }))
    .filter((group) => group.tutorials.length > 0);
}

export function allTags(tutorials: Tutorial[]) {
  return Array.from(new Set(tutorials.flatMap((tutorial) => tutorial.data.tags))).sort((a, b) =>
    a.localeCompare(b, "zh-CN"),
  );
}
