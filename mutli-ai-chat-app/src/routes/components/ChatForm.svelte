<script lang="ts">
  import { Button } from "$lib/components/ui/button/index.js";
  import { Textarea } from "$lib/components/ui/textarea/index.js";
  import { invoke } from "@tauri-apps/api/core";

  interface Props {
    question: string;
    chatMessages: string[];
  }

  let { question, chatMessages }: Props = $props();

  async function getTest() {
    let chatMessage: string = await invoke("greet", { name: question });
    chatMessages.push(chatMessage);
    question = "";
  }
</script>

<div class="flex flex-col">
  <Textarea
    rows={7}
    class="resize-none overflow-y-auto max-h-42"
    value={question}
  />
  <div class="flex justify-end">
    <Button
      class="mt-2"
      onclick={async () => {
        await getTest();
      }}>ask</Button
    >
  </div>
</div>
