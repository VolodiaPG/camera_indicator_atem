export class Message {
  color: "success" | "danger" | "warning" | "info";
  text: string;
  timeout: number; // secs
}

export let messageGuard = (val: unknown): val is Message => {
  let ret = val !== undefined && val !== null;
  ret = ret && val["text"] !== undefined;
  ret = ret && val["timeout"] !== undefined;
  ret = ret && ["success", "danger", "warning", "info"].includes(val["air"]);
  return ret;
};
