<script lang="ts">
  import { Role, type PageState } from "$lib/components/ts/pageState";
  import { Spinner } from "$lib/components/ui/spinner";
  import * as Card from "$lib/components/ui/card/index.js";

  interface Props {
    pageState: PageState;
  }

  let { pageState }: Props = $props();
</script>

<div class="flex-1 overflow-y-auto prose max-w-none mb-3">
  {#each pageState.chat_history as chat_history}
    {#if chat_history.role === Role.User}
      <div class="mx-3 my-2 font-semibold text-xl">
        {@html chat_history.chat}
      </div>
    {:else if chat_history.role === Role.ChatGPT}
      <div class="mx-3 my-2 text-left">
        {@html chat_history.chat}
      </div>
    {/if}
  {/each}

  {#if pageState.is_loading}
    <div class="flex items-center gap-4 px-3">
      <h4 class="animate-pulse">回答作成中</h4>
    </div>
  {/if}
</div>
