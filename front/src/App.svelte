<script lang="ts">
  import "bootstrap/dist/css/bootstrap.min.css";

  import { onMount } from "svelte";
  import InfoDialog from "./InfoDialog.svelte";
  import type { CameraAtem } from "./models";

  let status: CameraAtem;
  let socket: WebSocket;

  let camera_id;

  onMount(async () => {
    const data = await fetch("/api/register");
    console.log(data);

    status = {...((await data.json()) as unknown) as CameraAtem}; // TODO Change this thing !!!
  });

  let open_websocket = (url: string) => {
    let socket = new WebSocket(url);

    socket.onopen = function (e) {
      console.log("[open] Connection established");
    };

    socket.onmessage = function (event) {
      console.log(`[message] Data received from server: ${event.data}`);
      status = {...JSON.parse(event.data)};
    };

    socket.onclose = function (event) {
      if (event.wasClean) {
        console.log(
          `[close] Connection closed cleanly, code=${event.code} reason=${event.reason}`
        );
      } else {
        // e.g. server process killed or network down
        // event.code is usually 1006 in this case
        console.warn("[close] Connection died");
      }
    };

    socket.onerror = function (error) {
      console.warn(`[error] ${error}`);
    };

    return socket;
  };

  let register_new_ws = async (camera_id) => {
    const data = await fetch(`/api/register`, {
      method: "POST",
      headers: {
        Accept: "application/json",
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ camera_id: camera_id }),
    });

    const json = await data.json();
    let ws_url = json["url"];

    console.log("Connecting to url ", ws_url);
    
    socket?.close();
    socket = open_websocket(ws_url);
  };

  $: {
    // if the id changes
    void register_new_ws(camera_id);
  }
</script>

<main>
  <h1>Preview => {status?.preview}</h1>
  <h1>Air => {status?.air}</h1>
  <h1>Id => {camera_id}</h1>
  <svelte:component this={InfoDialog} bind:camera_id />
</main>

<svelte:head>
  <style>
    body {
      background-color: black;
    }
  </style>
</svelte:head>

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
