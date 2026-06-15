import { defineConfig } from "astro/config";
import mdx from "@astrojs/mdx";
import tailwindcss from "@tailwindcss/vite";

export default defineConfig({
  integrations: [mdx()],
  vite: {
    plugins: [tailwindcss()],
  },
  markdown: {
    shikiConfig: {
      // Code blocks are always rendered dark for consistent contrast,
      // independent of the site's light/dark theme toggle.
      theme: "github-dark",
      wrap: false,
    },
  },
});
