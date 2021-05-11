<script lang="ts">
  import { Input } from "sveltestrap";
  import { camera_id as ws_cam_id } from "../stores/websocket.store";

  let camera_id_str: string;
  export let camera_id: number = -1;

  $: {
    const parsed = parseInt(camera_id_str);
    if (!isNaN(parsed)) {
      camera_id = parsed;
    }
  }
  $: ws_cam_id.set(camera_id);
</script>

<main>
  <Input
    type="number"
    bind:value={camera_id_str}
    placeholder="number placeholder"
  />
</main>

<style>
</style>
