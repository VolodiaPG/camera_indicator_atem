import { writable } from "svelte/store";

const min_id = 1,
  max_id = 8;

// Writable to connect to the id
export const camera_id = writable<number>(10);

// Readable to output tge received messages
export const last_message = writable({});

// Ws
let socket: WebSocket;

// builds the WS and write to the store the new value received
let build_websocket = (url: string) => {
  let socket = new WebSocket(url);

  socket.onopen = function (e) {
    console.log("[open] Connection established");
  };

  socket.onmessage = function (event) {
    console.log(`[message] Data received from server: ${event.data}`);
    last_message.set(JSON.parse(event.data));
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

// Opens the websocket after getting an id from the backend
let open_websocket = async (camera_id: number) => {
  console.log(camera_id);

  if (camera_id < min_id || camera_id > max_id) {
    return;
  }

  const data = await fetch(`/api/register`, {
    method: "POST",
    headers: {
      Accept: "application/json",
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ camera_id: camera_id }),
  });

  const json = await data.json();
  let url = json["url"];

  console.log("Connecting to url ", url);

  socket?.close();
  socket = build_websocket(url);
};

// debounce
let timer;

const debounce = (v) => {
  clearTimeout(timer);
  timer = setTimeout(() => {
    open_websocket(+v);
  }, 750);
};

// updates the websocket after debounce
camera_id.subscribe((cam_id) => debounce(cam_id));
