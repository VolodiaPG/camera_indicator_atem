<script lang="ts">
  import { onDestroy } from "svelte";

  import {
    Input,
    Label,
    Card,
    CardBody,
    CardHeader,
    Button,
    Row,
    Container,
    Col,
  } from "sveltestrap";
  import { cameraAtemGuard } from "../models/CameraAtem.model";
  import type { Message } from "../models/Message.model";

  import { last_message } from "../stores/websocket.store";
  export let camera_id: number;
  let message: string;
  let timeout: string = "4";
  let cam_status: string = "⚫";

  const unsubscribe = last_message.subscribe((val: unknown) => {
    if (cameraAtemGuard(val)) {
      if (camera_id <= 0) {
        cam_status = "📢";
        return;
      }
      if (val.air === camera_id) {
        cam_status = "🔴";
        return;
      } else if (val.preview === camera_id) {
        cam_status = "🟢";
        return;
      }
      cam_status = "⚫";
    }
  });

  onDestroy(unsubscribe);

  const send_message = () => {
    const message_to_send: Message = {
      color: "info",
      timeout: parseInt(timeout),
      text: message,
    };

    const message_wrapper = {
      topic: "atem",
      message: JSON.stringify(message_to_send),
    };

    if (camera_id > 0) {
      message_wrapper["camera_id"] = camera_id;
    }

    fetch(`/api/publish`, {
      method: "POST",
      headers: {
        Accept: "application/json",
        "Content-Type": "application/json",
      },
      body: JSON.stringify(message_wrapper),
    }).then((_) => {
      message = undefined;
    });
  };

  const get_title = (camera_id) => {
    return camera_id > 0 ? camera_id : "Broadcast";
  };

  const on_key_press = (e: { charCode }) => {
    if (e.charCode === 13) send_message();
  };
</script>

<main>
  <Card>
    <CardHeader
      >{cam_status}
      {get_title(camera_id)}</CardHeader
    >
    <CardBody>
      <div>
        <Label>Message</Label>
        <div class="input-group">
          <Input plaintext bind:value={message} on:keypress={on_key_press} />
          <Input
            plaintext
            type="number"
            bind:value={timeout}
            on:keypress={on_key_press}
          />
        </div>
        <Button on:click={send_message}>Send</Button>
      </div></CardBody
    >
  </Card>
</main>

<style>
</style>
