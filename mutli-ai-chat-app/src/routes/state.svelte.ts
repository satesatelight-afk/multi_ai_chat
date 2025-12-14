import { readyMarkdownRenderer } from "$lib/components/ts/markdown";
import type MarkdownIt from "markdown-it";
import { onMount } from "svelte";

export let md: MarkdownIt | null = null;

onMount(async () => {
  md = await readyMarkdownRenderer();
});
