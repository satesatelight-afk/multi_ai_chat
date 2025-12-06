<script lang="ts">
  let { chatMessages } = $props();
</script>

<div class="flex-1 overflow-y-auto">
  {#each chatMessages as msg}
    {@html msg}
  {/each}
</div>
