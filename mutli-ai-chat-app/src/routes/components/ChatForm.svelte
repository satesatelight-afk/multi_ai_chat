<script lang="ts">
  import {
    Role,
    role_to_string,
    type PageState,
  } from "$lib/components/ts/pageState";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Spinner } from "$lib/components/ui/spinner";
  import { Textarea } from "$lib/components/ui/textarea/index.js";
  import { invoke } from "@tauri-apps/api/core";
  import type MarkdownIt from "markdown-it";

  interface Props {
    pageState: PageState;
    md: MarkdownIt | undefined;
  }

  let { pageState = $bindable(), md }: Props = $props();

  async function chatOpenAI() {
    if (md) {
      const question = pageState.textarea_question;
      pageState.diplay_question = question;
      Object.assign(pageState, {
        is_loading: true,
        display_question: question,
        textarea_question: "",
      });

      const question_history_chat = md.render(question);

      pageState.chat_history.push({
        role: Role.User,
        chat: question_history_chat,
      });

      const answer = await invoke<string>("chat_open_ai", {
        input: pageState.chat_history
          .map((c) => role_to_string(c.role) + ":" + c.chat)
          .join("\n"),
      });

      const answer_history_chat = md.render(answer);

      pageState.chat_history.push({
        role: Role.ChatGPT,
        chat: answer_history_chat,
      });

      Object.assign(pageState, {
        is_loading: false,
        display_question: "",
      });
    }
  }
</script>

<div class="flex flex-col">
  <Textarea
    rows={7}
    class="resize-none overflow-y-auto max-h-42"
    bind:value={pageState.textarea_question}
  />
  <div class="flex justify-end">
    {#if pageState.is_loading}
      <Button class="mt-2 disabled">
        <Spinner class="size-8" />
      </Button>
    {:else}
      <Button
        class="mt-2"
        onclick={() => {
          chatOpenAI();
        }}>ask</Button
      >
    {/if}
  </div>
</div>
