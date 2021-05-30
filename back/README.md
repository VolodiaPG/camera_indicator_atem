# warp-websockets-example
Basic example using websockets with warp in Rust

Run with

```bash
make dev
```

Then, you can register/unregister a client:

```bash
curl -X POST 'http://localhost:8000/register' -H 'Content-Type: application/json' -d '{ "user_id": 1 }' 

curl -X DELETE 'http://localhost:8000/register/e2fa90682255472b9221709566dbceba' 
```

Or connect to the WebSocket using the returned URL: `ws://127.0.0.1:8000/ws/625ac78b88e047a1bc7b3f8459702078`.

Then, you can publish messages using

```bash
curl -X POST 'http://localhost:8000/publish' \
    -H 'Content-Type: application/json' \
    -d '{"user_id": 1, "topic": "cats", "message": "are awesome"}'
```

## ATEM

To start the ATEM, the script `atem.ps1` needs to be called from and admin powershell using the command `powershell -Command .\atem.ps1` (type this inside the powershell). it executes powershell to recreate a new environment.

## Compile for release

1. Add the wanted targets
    ```sh
    rustup target add x86_64-pc-windows-gnu
    ```
2. Compile the wanted targets
    ```sh
    cd..
    make prod
    ```
3. Retrieve the file at `target/x86_64-pc-windows-gnu/release`

