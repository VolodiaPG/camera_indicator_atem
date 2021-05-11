<script lang="ts">
  import "bootstrap/dist/css/bootstrap.min.css";
  import "./global.css";
  import Main from "./pages/Main.svelte";
</script>

<main>
  <svelte:component this={Main} />
</main>

<svelte:head>
  <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap-icons@1.4.1/font/bootstrap-icons.css">
</svelte:head>
