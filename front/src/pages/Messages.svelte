<script lang="ts">
  import { onMount } from "svelte";

  import { last_message, camera_id } from "../stores/websocket.store";
  onMount(() => camera_id.set(0));

</script>

<main />

<style>
</style>
