<script lang="ts">
  import "bootstrap/dist/css/bootstrap.min.css";
  import "./global.css";
  import { Button, Icon } from "sveltestrap";
  import router, { page } from "./router";
  import {
    wakelock_fullscreen,
    fullscreen,
  } from "./stores/fullscreen_weblock.store";

  router.start();
</script>

<main>
  <svelte:component this={$page.component} {...$page.props} />
  <div class="top-right m-4">
    <Button on:click={wakelock_fullscreen}
      ><Icon name={$fullscreen ? "fullscreen-exit" : "fullscreen"} /></Button
    >
  </div>
</main>

<svelte:head>
  <link
    rel="stylesheet"
    href="https://cdn.jsdelivr.net/npm/bootstrap-icons@1.4.1/font/bootstrap-icons.css"
  />
</svelte:head>

<style>
  main {
    width: 100%;
    height: 100%;
    padding: 0;
    margin: 0;
  }

  .top-right {
    position: fixed;
    top: 0;
    right: 0;
  }
</style>
