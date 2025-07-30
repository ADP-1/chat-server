# Chat Application – Requirement Analysis

## 1. Introduction

This document defines the requirement analysis for a **C++ multi-threaded WebSocket-based Chat Application**. The application allows clients on the same network to join a common chatroom by connecting to a server via a specific port. Communication is real-time, and the server handles message broadcasting, user management, and graceful shutdown.

---

## 2. Purpose

The purpose of this document is to gather, analyze, and define the high-level requirements of the chat application to ensure correct implementation and alignment with the expected behavior. It will serve as a reference for all stakeholders during development.

---

## 3. Scope

* The application runs on LAN over a specified port.
* Server handles multiple clients using multi-threading.
* Clients join by providing a username; the server assigns them a 3-bit incremental ID.
* Messages are broadcasted to all connected clients.
* Handles malformed packets and client disconnection gracefully.
* Graceful server shutdown includes notifying all clients and closing resources.

---

## 4. Functional Requirements

### FR1: Client Join

* Input: Username
* Output: Welcome message, user added to chatroom
* Server assigns unique 3-bit ID

### FR2: Message Broadcast

* Messages sent by one client are received by all others
* Format: JSON (contains user, ID, message, timestamp)

### FR3: Disconnection Handling

* Clients receive notification when a user leaves

### FR4: Server Shutdown

* Graceful shutdown on command or signal
* Broadcast shutdown message
* Close sockets and threads safely

### FR5: User List Management

* Maintain active clients in memory using `std::map`

---

## 5. Non-Functional Requirements

### NFR1: Performance

* Must support at least 10 concurrent clients
* Thread-safe message delivery

### NFR2: Reliability

* Robust to malformed messages
* Does not crash on abrupt client exit

### NFR3: Maintainability

* Modular code structure
* Logging for debugging

### NFR4: Portability

* Primarily developed for Windows using Winsock, but may consider portability to Boost.Beast for cross-platform support

### NFR5: Security

* Basic input validation
* Prevent message spoofing

---

## 6. Assumptions

* All clients are on the same local network.
* Server machine IP and port are known to the clients.
* No file transfer or encryption support at this stage.

---

## 7. Constraints

* Uses C++ and Winsock for networking
* Single port only
* No GUI – command line only

---

## 8. Future Enhancements (Optional)

* GUI-based client
* Authentication system
* Cross-network communication
* Chat history persistence
* User roles (admin, user)

---

## 9. Glossary

| Term   | Description                                   |
| ------ | --------------------------------------------- |
| Client | A user program that connects to the server    |
| Server | Central process managing chat logic           |
| JSON   | Message format used for communication         |
| ID     | 3-bit unique identifier assigned to each user |

---
