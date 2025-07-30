# ✅ Secure LAN Chat Server – Project Implementation Plan

This plan outlines a **5-phase approach** to developing a secure LAN-based chat server using Rust and Axum. Each phase represents a weekly milestone and ends in a clearly defined deliverable.

---

## [ ] Phase 1: Project Setup & Core Server Foundation (Week 1)

**Goal:** Establish the foundational Rust project and get a basic Axum web server running that can serve static client-side files.

### Key Tasks:

* [ ] **Initialize Project:** Create a new Rust binary project using `cargo new chat-server`
* [ ] **Setup .gitignore:** Add the standard Rust `.gitignore` template to the project root
* [ ] **Add Core Dependencies:** Edit `Cargo.toml` to include:

  * `tokio`
  * `axum`
  * `tracing`
  * `tracing-subscriber`
* [ ] **Create Basic Server:** In `src/main.rs`, write code to start a basic Axum server on port 8080
* [ ] **Static File Serving:**

  * [ ] Add the `tower-http` dependency
  * [ ] Create a `wwwroot` directory
  * [ ] Place placeholder `index.html` and `client.js` in `wwwroot`
  * [ ] Configure Axum to serve files from `wwwroot`

✅ **Outcome:** Open `http://localhost:8080` and see the placeholder `index.html` page.

---

## [ ] Phase 2: WebSocket Connections & Real-Time Broadcasting (Week 2)

**Goal:** Add WebSocket communication and shared server state to enable real-time message broadcasting.

### Key Tasks:

* [ ] **WebSocket Handler:** Add a `/chat` endpoint to upgrade to WebSocket
* [ ] **Shared State (AppState):**

  * [ ] Create a `state.rs` module
  * [ ] Define `AppState` with a thread-safe list of clients (`Arc<Mutex<HashMap<...>>>`)
  * [ ] Pass `AppState` to Axum handlers
* [ ] **Connection Management:**

  * [ ] Add new clients to `AppState` on connect
  * [ ] Remove clients on disconnect
* [ ] **Broadcast Logic:** When a client sends a message, forward it to all others
* [ ] **Basic Client Logic:** Update `client.js` to:

  * [ ] Connect via WebSocket
  * [ ] Log received messages to the browser console

✅ **Outcome:** Multiple clients (browser tabs) can chat in real-time using WebSockets, shown via browser console.

---

## [ ] Phase 3: Client-Side UI & Chat History (Week 3)

**Goal:** Build the front-end UI and add server-side message history.

### Key Tasks:

* [ ] **Develop `index.html`:** UI with:

  * [ ] Message display area
  * [ ] Text input field
  * [ ] “Send” button
* [ ] **Develop `client.js`:**

  * [ ] Prompt user for username on load
  * [ ] Send username as first message
  * [ ] On send button click, transmit message to server
  * [ ] Display received messages in chat window
* [ ] **Server-Side History:**

  * [ ] Add a `Vec<String>` to `AppState` for last 100 messages
  * [ ] Implement FIFO logic to discard old messages
  * [ ] Send chat history to new clients upon connecting

✅ **Outcome:** Fully working chat app with persistent UI and limited message history.

---

## [ ] Phase 4: Configuration, Security (TLS), and Error Handling (Week 4)

**Goal:** Make the server secure, configurable, and robust.

### Key Tasks:

* [ ] **Configuration Support:**

  * [ ] Add `serde` and `config` crates
  * [ ] Create `config.toml` to manage settings (port, TLS paths, etc.)
  * [ ] Load configuration at server startup
* [ ] **Enable TLS:**

  * [ ] Generate self-signed TLS certs for local use
  * [ ] Use `axum-server` or equivalent for HTTPS/WSS
  * [ ] Update `client.js` to use `wss://`
* [ ] **Error Handling:**

  * [ ] Create custom `Error` type
  * [ ] Refactor handlers with `Result` and `?` for propagation
  * [ ] Show client-friendly error messages (e.g., “Unable to connect”)

✅ **Outcome:** HTTPS/WSS enabled server with external config and graceful error handling.

---

## [ ] Phase 5: Testing, Documentation, and Final Polish (Week 5)

**Goal:** Finalize the application for usability and reliability.

### Key Tasks:

* [ ] **Testing:**

  * [ ] Write unit tests for core logic (state, broadcasting)
  * [ ] Write integration tests simulating multiple clients
* [ ] **Manual QA:**

  * [ ] Test across browsers (Chrome, Firefox, Safari)
* [ ] **Documentation:**

  * [ ] Create `README.md` with:

    * [ ] Project overview
    * [ ] Configuration guide
    * [ ] Build & run instructions
* [ ] **Code Cleanup:**

  * [ ] Refactor for clarity
  * [ ] Apply Rust best practices
  * [ ] Remove unused code

✅ **Outcome:** Clean, documented, and stable LAN chat server ready for use or deployment.

