<script lang="ts">
  import { Input } from "sveltestrap";

  export let camera_id: Number;

  // debounce
  let val = "";
  let timer;

  const debounce = (v) => {
    clearTimeout(timer);
    timer = setTimeout(() => {
        camera_id = +v;
    }, 750);
  };
</script>

<main>
  <Input
    type="number"
    on:keyup={({ target: { value } }) => debounce(value)}
    placeholder="number placeholder"
  />
</main>

<style>
</style>
