<script lang="ts">
  import "bootstrap/dist/css/bootstrap.min.css";

  import InfoDialog from "../components/InfoDialog.svelte";
  import type { CameraAtem } from "../models/CameraAtem.model";

  let status: CameraAtem;

  let camera_id;

  // onMount(async () => {
  //   const data = await fetch("/api/register");
  //   console.log(data);

  //   status = { ...(((await data.json()) as unknown) as CameraAtem) }; // TODO Change this thing !!!
  // });

  // $: {
  //   // if the id changes
  //   void register_new_ws(camera_id);
  // }
</script>

<main>
  <h1>Preview => {status?.preview}</h1>
  <h1>Air => {status?.air}</h1>
  <h1>Id => {camera_id}</h1>
  <svelte:component this={InfoDialog} bind:camera_id />
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  h1 {
    color: #ff3e00;
    text-transform: uppercase;
    font-size: 4em;
    font-weight: 100;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
