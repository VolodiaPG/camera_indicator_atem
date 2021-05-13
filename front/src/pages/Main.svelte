<script lang="ts">
  import "bootstrap/dist/css/bootstrap.min.css";
  import { onDestroy } from "svelte";
  import InfoDialog from "../components/InfoDialog.svelte";
  import Notifications from "../components/Notifications.svelte";
  import { CameraAtem, cameraAtemGuard } from "../models/CameraAtem.model";
  import { last_message } from "../stores/websocket.store";

  let camera_id;

  let saved_message: CameraAtem;

  const get_atem = (obj: unknown): CameraAtem | undefined => {
    if (cameraAtemGuard(obj)) {
      saved_message = obj;
    }
    return saved_message;
  };

  // let wakeLock = null;

  // // keep screen on
  // if ("wakeLock" in navigator) {
  //   const isSupported = true;
  //   console.log("Screen Wake Lock API supported!");
  //   // Create a reference for the Wake Lock.

  //   // create an async function to request a wake lock
  //   try {
  //     wakeLock = await navigator.wakeLock.request("screen");
  //     console.log("Wake Lock is active!");
  //   } catch (err) {
  //     // The Wake Lock request has failed - usually system related, such as battery.
  //    console.warn(`${err.name}, ${err.message}`);
  //   }
  // } else {
  //   console.log.warn("Wake lock is not supported by this browser.");
  // }

  // onDestroy(() => {
  //   wakeLock.release().then(() => {
  //     wakeLock = null;
  //   });
  // });
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
