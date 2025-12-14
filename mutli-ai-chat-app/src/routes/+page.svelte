<script lang="ts">
  import { type PageState } from "$lib/components/ts/pageState";
  import { onMount } from "svelte";
  import ChatDisplay from "./components/ChatDisplay.svelte";
  import ChatForm from "./components/ChatForm.svelte";
  import { readyMarkdownRenderer } from "$lib/components/ts/markdown";
  import type MarkdownIt from "markdown-it";

  let md = $state<MarkdownIt | undefined>(undefined);

  let pageState = $state<PageState>({
    textarea_question: "",
    diplay_question: "",
    answer: "",
    chat_history: [],
    is_loading: false,
  });

  onMount(async () => {
    if (!md) {
      md = await readyMarkdownRenderer();
    }
  });
</script>

<div class="flex flex-col h-[calc(100vh-80px)]">
  <ChatDisplay {pageState} />
  <ChatForm bind:pageState {md} />
</div>
