<script lang="ts">
  import { onMount } from "svelte";
  import type { CameraAtem } from "./models";

  export let status: CameraAtem;

  onMount(async () => {
    const data = await fetch("/api/status");
    status = ((await data.json()) as unknown) as CameraAtem; // TODO Change this thing !!!
  });
</script>

<main>
  <h1>Preview => {status?.cam_preview}</h1>
  <h1>Prod => {status?.cam_prod}</h1>
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
