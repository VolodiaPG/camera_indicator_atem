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
  import { CameraAtem, cameraAtemGuard } from "../models/CameraAtem.model";
  import { camera_id as ws_cam_id } from "../stores/websocket.store";
  import { last_message } from "../stores/websocket.store";

  let open = true;
  let camera_id_str: string;
  export let camera_id: number = -1;

  const get_message = (obj: unknown): CameraAtem | undefined => {
    if (cameraAtemGuard(obj)) return obj;
    return undefined;
  };

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
  <Button outline color="secondary" on:click={toggle}
    ><Icon name="gear" />&nbsp;Settings</Button
  >
  <Modal isOpen={open} {toggle}>
    <ModalHeader {toggle}>Settings</ModalHeader>
    <ModalBody>
      <div>Preview: {get_message($last_message)?.preview}</div>
      <div>air: {get_message($last_message)?.air} </div>
      <div>my ID: {camera_id} </div>
      <Input type="number" bind:value={camera_id_str} placeholder="Camera ID" />
    </ModalBody>
    <ModalFooter>
      <Button color="secondary" on:click={toggle}>Close</Button>
    </ModalFooter>
  </Modal>
</main>

<style>
</style>
