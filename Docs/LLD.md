### \#\# **Low-Level Design (LLD) - Rust, Axum & Tokio**

**1. Module Responsibilities**

The codebase will be organized into several modules within the `src/` directory to ensure clear separation of concerns.

  * **`main.rs` (Application Entrypoint):**

      * Initializes logging/tracing (e.g., `tracing_subscriber`).
      * Loads configuration from a file or environment variables.
      * Creates the shared `AppState` and wraps it in an `Arc`.
      * Builds the `axum::Router`, defining routes for static files and the WebSocket endpoint.
      * Sets up a graceful shutdown handler for `Ctrl+C`.
      * Binds the TCP listener (with TLS if configured) and starts the server.

  * **`handlers.rs` (Route Handlers):**

      * Contains the asynchronous functions that handle incoming requests.
      * `websocket_handler()`: The primary handler for the `/chat` route. It accepts the `WebSocketUpgrade` extractor and the shared `State<Arc<AppState>>`. Its only job is to perform the handshake and spawn a new Tokio task by calling `handle_client_connection()`.
      * `handle_client_connection()`: This function runs within the spawned task for each connected client. It manages the entire client lifecycle:
        1.  Reads the initial "join" message to get the username.
        2.  Adds the client to the `AppState` and gets the chat history.
        3.  Sends the history to the new client.
        4.  Broadcasts the "join" notification.
        5.  Enters a loop, using `tokio::select!` to simultaneously listen for new messages from the client's WebSocket and messages from the server (via a broadcast channel).
        6.  Handles cleanup on disconnect.

  * **`state.rs` (Shared State):**

      * Defines the `AppState` struct.
      * Defines the `ChatState` struct, which is wrapped in a `tokio::sync::Mutex` inside `AppState`. This contains the message history (`Vec<String>`) and the map of connected clients (`HashMap<UserId, mpsc::Sender<String>>`).
      * Implements the `broadcast()` method, which locks the `ChatState` and iterates through the connected clients to send a message down each one's channel.

-----

**2. Data Structures**

  * **`AppState`:** The top-level shared state struct, wrapped in an `Arc` for shared ownership across tasks.
    ```rust
    // In state.rs
    pub struct AppState {
        pub chat_state: Mutex<ChatState>,
    }

    pub struct ChatState {
        // Limited to the last N messages
        history: Vec<String>,
        // Maps a unique user ID to a channel sender for that user
        sessions: HashMap<UserId, mpsc::Sender<String>>,
    }

    // A unique ID for each session
    type UserId = u128;
    ```
  * **Channels (`tokio::sync::mpsc`):** Each client task will have a multi-producer, single-consumer (mpsc) channel for receiving broadcasted messages from the server. The `Sender` half is stored in the `AppState`, and the `Receiver` half is owned by the client's task.
  * **Buffers:** Network and message buffering is handled internally and efficiently by Tokio and Axum's underlying Hyper library. We do not need to manage manual buffers.

-----

**3. Error Handling & Cleanup**

  * **Error Propagation:** Handlers will return `Result<T, E>`. The `?` operator will be used to cleanly propagate errors. Custom error types will be defined that can be converted into an Axum `Response`.
  * **RAII & `drop`:** Rust's ownership model ensures automatic cleanup. When a client task finishes (either by clean disconnect or error), its owned resources (like the WebSocket connection and the `Receiver` end of its channel) are automatically dropped. The `drop` implementation for our session handler will ensure the user is removed from the `AppState`'s session map.
  * **Graceful Shutdown:** `main.rs` will use `tokio::signal::ctrl_c` to listen for the shutdown signal. When received, it will call a shutdown hook on the Axum server, allowing active connections to terminate cleanly. A final "Server is shutting down" message can be broadcast.

-----

**4. Configuration & Constants**

Configuration will be loaded at startup from a `config.toml` file or environment variables using a crate like `serde` and `config`.

  * **Default Port:** `8080`
  * **Webroot Directory:** `"./wwwroot/"`
  * **Max History Size:** `100` (messages)
  * **TLS Configuration:** Paths to `cert.pem` and `key.pem` files (optional).

-----

**5. File Structure**

The project will follow the standard Rust crate structure.

```
chat-server/
├── Cargo.toml          # Project dependencies (axum, tokio, tracing, etc.)
├── Dockerfile          # For easy containerized deployment
├── config/
│   └── server.toml     # Default server configuration
└── wwwroot/
│   ├── index.html
│   └── client.js
└── src/
    ├── main.rs         # Entry point, router setup, server start
    ├── handlers.rs     # WebSocket and client connection handlers
    ├── state.rs        # AppState and ChatState structs and logic
    └── error.rs        # Custom error types for the application
```