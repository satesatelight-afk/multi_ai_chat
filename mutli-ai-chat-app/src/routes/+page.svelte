<script lang="ts">
  import ChatDisplay from "./components/ChatDisplay.svelte";
  import ChatForm from "./components/ChatForm.svelte";

  let question = $state<string>("");
  let chatMessages = $state<string[]>([]);
</script>

<div class="flex flex-col h-[calc(100vh-80px)]">
  <ChatDisplay {chatMessages} />
  <ChatForm {question} {chatMessages} />
</div>
