import { readyMarkdownRenderer } from "$lib/components/ts/markdown";
import type MarkdownIt from "markdown-it";
import { onMount } from "svelte";

export const md = await readyMarkdownRenderer();

// export let md: MarkdownIt | null = null;

// onMount(async () => {
//   if (!md) {
//     md = await readyMarkdownRenderer();
//   }
// });
