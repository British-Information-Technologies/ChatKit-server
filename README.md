# Rust-chat-server

A Chat server writen in rust to allow communication between peers.

---

## Features:
- implemented:
  - json based API.
  - Server introspection.
  - Peer discovery.
  - sending messages to connected clients.
  - 
- todo:
  - Encryption to server.
  - server to server meshing.
  - asynchronous client managment instead of threaded approach.

## Goals:
- Learn the rust programming lanaguage.
  - Ownership: how that affects normal programming styles.
  - Borrowing and references: how this affects shared state.
  - Lifetimes: how this affects data retention and sharing.
- Learn how to create networked programs.
  - Application level protocol: how to get two programs to communicate via TCP sockets.
  - Socket handling: Discovering ways to handle multiple socket connections without affecting performance.
- Learn common encryption protocols.
  - Adding support for encrypted sockets.
  - Pros and cons of symetric and asymetric encryption.
  - resolving common encryption flaws

> Questions: For questions please add a issue with the question label. It will eventually be responded to
