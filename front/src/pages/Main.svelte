<script lang="ts">
  import "bootstrap/dist/css/bootstrap.min.css";
  import { Button } from "sveltestrap";
  import InfoDialog from "../components/InfoDialog.svelte";
  import Notifications from "../components/Notifications.svelte";
  import { CameraAtem, cameraAtemGuard } from "../models/CameraAtem.model";
  import { last_message } from "../stores/websocket.store";
  import NoSleep from "nosleep.js";

  let camera_id;

  let saved_message: CameraAtem;

  const get_atem = (obj: unknown): CameraAtem | undefined => {
    if (cameraAtemGuard(obj)) {
      saved_message = obj;
    }
    return saved_message;
  };

  const noSleep = new NoSleep();

  const wakelock = () => {
    if (!noSleep.isEnabled){
      noSleep.enable();
    }else{
      noSleep.disable();
    }
  };
</script>

<main
  class:bg-danger={get_atem($last_message)?.air === camera_id}
  class:bg-success={get_atem($last_message)?.preview === camera_id}
>
  <div class="top-right m-4">
    <svelte:component this={InfoDialog} bind:camera_id />
    <Button on:click={wakelock}>Wakelock</Button>
  </div>
  <svelte:component this={Notifications} />
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    width: 100%;
    height: 100%;
    padding: 0;
    margin: 0;
    background-color: black;
  }

  .top-right {
    position: fixed;
    top: 0;
    right: 0;
  }
</style>
