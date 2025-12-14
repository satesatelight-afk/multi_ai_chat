// src/lib/markdown.ts
import MarkdownIt from "markdown-it";
import { fromHighlighter } from "@shikijs/markdown-it";
import { bundledLanguages, createHighlighter } from "shiki";

export async function readyMarkdownRenderer(): Promise<MarkdownIt> {
  const highlighter = await createHighlighter({
    themes: ["dracula-soft"],
    langs: Object.keys(bundledLanguages),
  });

  const md = new MarkdownIt({
    html: true,
    linkify: true,
    typographer: true,
  });

  md.use(
    fromHighlighter(highlighter, {
      theme: "dracula-soft",
      fallbackLanguage: "bash", // ★ 存在しない言語は txt 扱い
    })
  );

  return md;
}
