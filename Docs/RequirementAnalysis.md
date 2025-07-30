# Requirement Analysis: Secure LAN Chat Server  
**Version:** 1.1  
**Date:** July 30, 2025  

---

## 1. Introduction

### 1.1 Purpose  
This document outlines the functional and non-functional requirements for a secure, real-time, LAN-based chat application. The system will consist of a server backend developed in Rust and a web-based frontend accessible from any modern browser on the same local network. The primary purpose is to provide a private, fast, and reliable communication channel for users within a trusted network environment (e.g., an office, home, or event).

### 1.2 Scope  
The scope of this project (Version 1.1) is to deliver a functional group chat application. Key features include allowing multiple users to join a single chat room by providing a username, sending and receiving messages in real-time, and viewing a short history of recent messages. The system will prioritize security through encrypted transport and reliability through robust server-side logic. Features outside this scope, such as multiple rooms, private messaging, and persistent long-term storage, are designated as non-goals for this version.

### 1.3 Intended Audience & Use Case  
**Intended Audience:** Users on a shared local area network (LAN) who need to communicate without relying on internet-based services.  

**Use Case:** A team in an office needs a quick way to coordinate without using external tools. Attendees at a local event or LAN party want a private chat channel. Family members at home want a simple, shared message board.

---

## 2. Overall Description

### 2.1 Product Perspective  
The product is a self-contained client-server application. The server is a standalone executable that runs on a designated machine on the LAN. The client is a web application served by the server itself, requiring no installation on user machines other than a standard web browser.

### 2.2 User Characteristics  
Users are expected to have basic computer literacy and the ability to open a web browser and navigate to a local IP address. No special technical skills are required.

### 2.3 Assumptions & Dependencies  
- **Assumption:** All users are on the same stable and trusted local area network.  
- **Assumption:** Users' devices have a modern web browser that supports WebSockets.  
- **Dependency:** A single machine on the network must be designated to run the server executable continuously for the service to be available.

---

## 3. Functional Requirements

### FR-1: User Connection & Session Initiation  
- **FR-1.1:** The system shall allow a user to join the chat by navigating to the server's address in a web browser and entering a username.  
- **FR-1.2:** The system shall reject connection attempts with empty or whitespace-only usernames.  
- **FR-1.3:** Upon a successful join, the system shall broadcast a notification to all existing users (e.g., "Alice has joined.").  
- **FR-1.4:** The system shall permit multiple users to have the same username. Usernames serve as display labels only.

### FR-2: Message Exchange  
- **FR-2.1:** An authenticated user shall be able to type a text message and send it to the group chat.  
- **FR-2.2:** The system shall prepend the sender's username to the message before broadcasting (e.g., "Alice: Hello world!").  
- **FR-2.3:** The system shall deliver the message to all currently connected users in real-time.

### FR-3: Chat History  
- **FR-3.1:** The server shall maintain an in-memory history of the most recent messages (e.g., the last 100 messages).  
- **FR-3.2:** When a new user joins the chat, the system shall immediately send them the entire in-memory chat history to bring them up to date.  
- **FR-3.3:** When the message history limit is reached, the oldest message shall be discarded to make room for the new one (FIFO).

### FR-4: User Disconnection  
- **FR-4.1:** When a user closes their browser tab or disconnects, the system shall detect the termination of the connection.  
- **FR-4.2:** The system shall remove the user from the list of active sessions.  
- **FR-4.3:** The system shall broadcast a leave notification to all remaining users (e.g., "Alice has left.").

### FR-5: Client Application Serving  
- **FR-5.1:** The server shall respond to HTTP GET requests on the root path (/) by serving the static HTML, CSS, and JavaScript files required for the client application to run.

---

## 4. Non-Functional Requirements

### NFR-1: Performance  
- **NFR-1.1:** Message delivery latency between a sender and all receivers on the same LAN shall be less than 500 milliseconds under normal network conditions.  
- **NFR-1.2:** The server shall support at least 50 concurrent users without significant performance degradation.

### NFR-2: Security  
- **NFR-2.1:** All communication between the client browsers and the server shall be encrypted using TLS (HTTPS for the web page, WSS for the WebSocket). Certificates can be self-signed for trusted LAN environments.  
- **NFR-2.2:** The server must be robust against common WebSocket vulnerabilities and malformed data packets.  
- **NFR-2.3:** User identity is limited to a display name for the current session. No password authentication is implemented, and usernames are not guaranteed to be unique.

### NFR-3: Reliability  
- **NFR-3.1:** The server shall be designed to run continuously without crashing. In case of an error with a single client connection, it must not affect any other connected clients.  
- **NFR-3.2:** The server executable shall have a graceful shutdown mechanism to ensure clean termination and resource release.  
- **NFR-3.3:** The client UI must provide clear, non-technical feedback to the user in case of an error (e.g., "Invalid username" or "Could not connect to the server.").

### NFR-4: Usability  
- **NFR-4.1:** The client user interface shall be clean, intuitive, and require no instructions to use.  
- **NFR-4.2:** The chat history shall be scrollable, with new messages appearing at the bottom.

### NFR-5: Deployment  
- **NFR-5.1:** The entire server application shall be compiled into a single, standalone executable with no external runtime dependencies (other than the OS).  
- **NFR-5.2:** The server shall be configurable via a simple configuration file (e.g., `config.toml`) to set the port, webroot, and TLS certificate paths.  
- **NFR-5.3:** Any changes made to the configuration file require a server restart to take effect.
