# **Fabula: Actor-Driven Distributed Chatroom (Rust + Tokio)**

Fabula is a lightweight multi-user chat system built on an internal **Actor model**, powered by **Tokio**’s asynchronous runtime.
It features clean separation of concerns, simple extensibility, and a fully typed JSON protocol designed for terminal-based clients.

This project aims to serve as a clear, educational, and production-adjacent example of:

* building an actor system from scratch in Rust
* managing user sessions and chat rooms
* designing async message routing
* integrating an embedded key-value database (`sled`)
* handling structured protocols over TCP with length-delimited frames

### **Main Features**

* 🚀 **Async Server** using Tokio
* 🎭 **Custom Actor Framework** (Session / Room / Router / DB Actors)
* 💬 **Multi-Room Chat** with per-room isolation
* 🔐 **User Login & Registration** with persistent storage
* 📨 **Structured Client/Server Protocol** using JSON + length-prefix framing
* 🛠 **CLI Client** with interactive commands
* 📦 **Simple, Embedded Storage** powered by `sled`

### **Project Structure**

```
src/
  proto/        # Protocol definitions (messages, codec)
  server/       # Actor system, router, rooms, DB, network handling
  client/       # CLI client, REPL, connection logic
```

### **Status**

This is an ongoing project.
Core actor infrastructure, protocol, and networking foundations are being actively developed.

### **Goals**

* Provide a clear Actor-based architecture for async applications
* Offer an extendable baseline for future distributed chat or messaging tools

### **License**

MIT

