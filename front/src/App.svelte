<script lang="ts">
  import "bootstrap/dist/css/bootstrap.min.css";
  import "./global.css";

  import router, { page } from "./router";
  router.start();
</script>

<main>
  <svelte:component this={$page.component} {...$page.props} />
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
</style>
