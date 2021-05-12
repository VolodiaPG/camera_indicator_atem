<script lang="ts">
  import { onMount } from "svelte";
  import { Col, Container, Row, Card, CardBody, CardHeader } from "sveltestrap";
  import SenderMessageCard from "../components/SenderMessageCard.svelte";

  import { camera_id } from "../stores/websocket.store";
  onMount(() => {
    camera_id.set(1);
    // camera_id.set(0);
  });
</script>

<main>
  <Container>
    <div class="m-2">
      <svelte:component this={SenderMessageCard} camera_id=0 />
    </div>
    <Row>
      {#each { length: 8 } as _, ii}
        <Col xs="12" sm="6" lg="4">
          <div class="m-2">
            <svelte:component this={SenderMessageCard} camera_id={ii + 1} />
          </div>
        </Col>
      {/each}
    </Row>
  </Container>
</main>

<style>
</style>
