import { defineCollection, z } from "astro:content";
import { glob } from "astro/loaders";

const stageSchema = z.enum([
  "intro",
  "basic",
  "ownership",
  "modeling",
  "std",
  "generics",
  "iterators",
  "smart-pointers",
  "concurrency",
  "crate",
  "quality",
  "async",
  "ecosystem",
  "web-db",
  "advanced",
  "project",
]);

const tutorials = defineCollection({
  loader: glob({ pattern: "**/*.mdx", base: "./src/content/tutorials" }),
  schema: z.object({
    title: z.string(),
    description: z.string(),
    stage: stageSchema,
    order: z.number().int().nonnegative(),
    level: z.enum(["beginner", "intermediate", "advanced"]),
    estimatedMinutes: z.number().int().positive(),
    tags: z.array(z.string()).default([]),
    rustConcepts: z.array(z.string()).default([]),
    prerequisites: z.array(z.string()).default([]),
    exampleDir: z.string().optional(),
    draft: z.boolean().default(false),
    updatedAt: z.string().regex(/^\d{4}-\d{2}-\d{2}$/),
  }),
});

export const collections = { tutorials };
