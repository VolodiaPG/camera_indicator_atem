<script lang="ts">
  import { onDestroy } from "svelte";
  import { last_message } from "../stores/websocket.store";
  import { messageGuard, Message } from "../models/Message.model";
  import SimpleAlert from "./SimpleAlert.svelte";

  let message: Message;

  const unsubscribe = last_message.subscribe((val: unknown) => {
    if (messageGuard(val)) {
      console.log(val);

      message = val;
    }
  });

  onDestroy(unsubscribe);
</script>

<main>
  <div class="fixed-bottom m-5 message-text">
    <svelte:component this={SimpleAlert} {message} />
  </div>
</main>

<style>
  .message-text {
    font-size: 15vh;
  }
</style>
