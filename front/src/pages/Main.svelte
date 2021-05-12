<script lang="ts">
  import "bootstrap/dist/css/bootstrap.min.css";
  import InfoDialog from "../components/InfoDialog.svelte";
  import Notifications from "../components/Notifications.svelte";
  import { CameraAtem, cameraAtemGuard } from "../models/CameraAtem.model";
  import { last_message } from "../stores/websocket.store";

  let camera_id;

  const get_atem = (obj: unknown): CameraAtem | undefined => {
    if (cameraAtemGuard(obj)) return obj;
    return undefined;
  };
</script>

<main
  class:bg-danger={get_atem($last_message)?.air === camera_id}
  class:bg-success={get_atem($last_message)?.preview === camera_id}
>
  <div class="top-right m-4">
    <svelte:component this={InfoDialog} bind:camera_id />
  </div>
  <svelte:component this={Notifications} />
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
    background-color: black;
  }

  .top-right {
    position: fixed;
    top: 0;
    right: 0;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
