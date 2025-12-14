<script lang="ts">
  import { type PageState } from "$lib/components/ts/pageState";
  import ChatDisplay from "./components/ChatDisplay.svelte";
  import ChatForm from "./components/ChatForm.svelte";

  let pageState = $state<PageState>({
    textarea_question: "",
    diplay_question: "",
    answer: "",
    chat_history: [],
    is_loading: false,
  });
</script>

<div class="flex flex-col h-[calc(100vh-80px)]">
  <ChatDisplay {pageState} />
  <ChatForm bind:pageState />
</div>
