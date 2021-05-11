<script lang="ts">
  import {
    Button,
    Modal,
    ModalBody,
    ModalFooter,
    ModalHeader,
    Input,
Icon,
  } from "sveltestrap";
  import { camera_id as ws_cam_id } from "../stores/websocket.store";

  let open = true;
  let camera_id_str: string;
  export let camera_id: number = -1;

  const toggle = () => (open = !open);

  $: {
    const parsed = parseInt(camera_id_str);
    if (!isNaN(parsed)) {
      camera_id = parsed;
    }
  }
  $: ws_cam_id.set(camera_id);
</script>

<main>
  <Button outline color="secondary" on:click={toggle}><Icon name="gear"/>&nbsp;Settings</Button>
  <Modal isOpen={open} {toggle}>
    <ModalHeader {toggle}>Settings</ModalHeader>
    <ModalBody>
      <Input
        type="number"
        bind:value={camera_id_str}
        placeholder="Camera ID"
      />
    </ModalBody>
    <ModalFooter>
      <Button color="secondary" on:click={toggle}>Close</Button>
    </ModalFooter>
  </Modal>
</main>

<style>
</style>
