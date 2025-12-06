<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  import { Button } from "$lib/components/ui/button/index.js";
  import { Textarea } from "$lib/components/ui/textarea/index.js";

  let name = $state("");
</script>

<div class="flex flex-col h-[calc(100vh-80px)]">
  <div class="flex-1"></div>
  <div class="flex flex-col">
    <Textarea rows={5} maxlength={5}></Textarea>
    <div class="flex justify-end">
      <Button class="mt-2">ask</Button>
    </div>
  </div>
</div>
