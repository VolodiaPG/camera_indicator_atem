<script lang="ts">
  import "bootstrap/dist/css/bootstrap.min.css";
  import { onDestroy } from "svelte";
  import InfoDialog from "../components/InfoDialog.svelte";
  import { CameraAtem, cameraAtemGuard } from "../models/CameraAtem.model";
  import { last_message } from "../stores/websocket.store";

  let status: CameraAtem;

  const unsubscribe = last_message.subscribe((value) => {
    if (cameraAtemGuard(value)) {
      status = value;
    }
  });

  let camera_id;

  onDestroy(unsubscribe);
</script>

<main
  class:bg-danger={status?.air === camera_id}
  class:bg-success={status?.preview === camera_id}
>
  <div class="fixed-top m-1 top-bar text-secondary">
    <span>Preview: {status?.preview}</span>
    <span class="sep" />
    <span>air: {status?.air} </span>
    <span class="sep" />
    <span>my ID: {camera_id} </span>
  </div>
  <div class="bottom-right m-4">
    <svelte:component this={InfoDialog} bind:camera_id />
  </div>
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
    background-color: black;
  }

  .sep {
    margin-left: 2em;
  }
  .top-bar {
    color: #ff3e00;
    text-transform: uppercase;
    font-size: 1em;
  }

  .bottom-right {
    position: fixed;
    bottom: 0;
    right: 0;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
