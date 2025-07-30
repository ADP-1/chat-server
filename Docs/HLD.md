### ## **High-Level Design (HLD) - Rust & Axum**

**1. Overview**

A LAN-based browser chat server implemented in **Rust** using the **Tokio** asynchronous runtime and the **Axum** web framework. The server provides a robust HTTP/S interface for serving static client assets (HTML, JavaScript) and upgrades connections to the secure WebSocket (WSS) protocol for real-time, memory-safe messaging.

---

**2. Goals & Non-Goals**

* **Goals:**
    * Real-time group chat over a local network.
    * **Memory and Thread Safety by default**, enforced by the Rust compiler.
    * **Secure Transport (TLS/WSS)** as a standard feature, not an afterthought.
    * Browser-based client requiring only HTML and JavaScript.
    * Simple username-based join flow.
    * Broadcast messaging with join/leave notifications.
    * A single, statically-linked executable for easy deployment.

* **Non-Goals (v1):**
    * Persistent chat history storage.
    * User authentication beyond entering a username.
    * Private or one-to-one messaging, or multiple chat rooms.

---

**3. System Components**

* **Main Application (`main.rs`):**
    * The entry point, running on the **Tokio** async runtime (`#[tokio::main]`).
    * Initializes the shared application state (`AppState`).
    * Defines the **Axum `Router`**, which maps URL paths to handler functions.
    * Binds a TCP listener to a configured port (e.g., 8080) and serves the application, including TLS setup.

* **Router and Handlers:**
    * **Static File Service:** A route (`/`) that uses the `tower_http::services::ServeDir` service to efficiently serve all files from the `wwwroot` directory. This handles all GET requests for HTML, JS, and CSS files without custom logic.
    * **WebSocket Handler (`/chat`):** An asynchronous Axum handler function that accepts a `WebSocketUpgrade` request. It performs the handshake to establish a WebSocket connection and then **spawns a new Tokio task** to manage the entire lifecycle of that single client connection.

* **Shared State (`AppState`):**
    * A central `struct` that holds the shared data for the entire application.
    * It contains a thread-safe collection of active client sessions, such as a `Mutex<HashMap<UserId, Sender>>`, where `Sender` is a channel to send messages to a specific client's task.
    * The entire `AppState` is wrapped in an `Arc` (Atomically Reference Counted pointer) to be shared safely and efficiently across all handlers and tasks without data races.

---

**4. Data Flow**

1.  A browser requests the root path (`/`). The **Axum Router** directs the request to the **Static File Service**, which responds with the `index.html` and `client.js` files.
2.  The client-side JavaScript establishes a secure WebSocket connection to the `/chat` endpoint.
3.  The **WebSocket Handler** recognizes the upgrade request, completes the handshake, and spawns a new, isolated **Tokio task** to manage the connection.
4.  This new task reads the first message from the client to get the username, registers the client in the shared **`AppState`** (by locking the `Mutex`), and then broadcasts a "user joined" notification to all other registered clients.
5.  Subsequent text messages from the client are received within its dedicated task. The task then locks the `AppState` to broadcast the message to all other participants.
6.  On client disconnect or error, the task automatically cleans up. Its `drop` implementation ensures the client is removed from the `AppState` and a "user left" notification is broadcast.

---

**5. Interaction Diagram (Textual)**

The external API contract remains the same as the original design.

* **Client to Server (HTTP):** `GET /` → server responds with static files.
* **Client to Server (WebSocket Upgrade):** `GET /chat` with upgrade headers → server completes handshake.
* **Client to Server (Join):** WebSocket send username → server registers session and broadcasts join message.
* **Client to Server (Message):** WebSocket send text message → server broadcasts message to all.
* **Client to Server (Disconnect):** Connection close or error → server deregisters session and broadcasts leave message.