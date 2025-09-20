# Whiteboard

This is a collaborative whiteboard with real-time synchronization via a Socket.IO server written in Rust.  
The client uses [Excalidraw](https://excalidraw.com/) and is served directly by the Rust backend.

## Features

-   Real-time collaboration
-   Reliable synchronization using a Rust Socket.IO backend
-   User-friendly Excalidraw interface

## Example Interface

![Whiteboard Example](https://github.com/webshining/whiteboard/raw/main/example.png)

## Getting Started

1. **Start the Rust server**  
   In the project directory, run:

    ```sh
    cd frontend
    npm i
    npm run build

    cd ../
    cargo run
    ```

    The server will serve both the backend and the Excalidraw-based client.

2. **Access the Whiteboard**  
   Open your browser and go to the address shown in the server output `http://localhost:4000`.

Make sure the server is running for full functionality.
