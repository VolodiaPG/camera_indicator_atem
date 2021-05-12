<script lang="ts">
  import { onMount } from "svelte";

  import { Alert } from "sveltestrap";
  import type { Message } from "../models/Message.model";

  export let message: Message;
  let displayed_message: Message | undefined;
  let messages: Message[] = [];
  let timer: number;

  onMount(() => {
    timer = -1;
    messages = [];
  });

  function create_timer(): void {
    if (messages.length === 0) {
      displayed_message = undefined;
      timer = -1;
      return;
    }

    if (timer < 0) {
      displayed_message = messages.pop();
      timer = setTimeout(() => {
        displayed_message = undefined;
        timer = -1;
        setTimeout(() => {
          create_timer();
        }, 250);
      }, displayed_message.timeout * 1000);
    }
  }

  // push the new value on change
  $: {
    if (message !== undefined) {
      messages.push(message);

      create_timer();
    }
  }
</script>

<main>
  {#if displayed_message !== undefined}
    <Alert color={displayed_message.color}>
      <span>{displayed_message.text}</span>
    </Alert>
  {/if}
</main>

<style>
</style>
