# DashDB ⚡

DashDB is a high-performance, asynchronous, in-memory key-value datastore built entirely in Rust. Designed with low-latency system design principles in mind, it operates over raw TCP using a custom, lightweight binary protocol. 

It leverages `tokio` for asynchronous non-blocking I/O and `DashMap` for highly concurrent, thread-safe sharded state management.

## 🚀 Key Features

- **Custom Binary Protocol**: Avoids HTTP overhead by utilizing a lightweight, custom TCP packet structure for maximum throughput.
- **Highly Concurrent**: Utilizes `DashMap` for lock-free read/write operations across multiple threads, ensuring the database scales with CPU cores.
- **Asynchronous I/O**: Built on the `tokio` runtime, enabling the server to handle thousands of concurrent TCP connections efficiently.
- **Token-based Authentication**: Secure connection handshakes require an authentication token before accepting operations.
- **Graceful Shutdown**: Intercepts `SIGTERM` and `SIGINT` signals to safely drain and close active connections.
- **Docker Ready**: Multi-stage, minimal Alpine Linux Dockerfile and `docker-compose` setup for immediate deployment.

---

## 🏗️ Architecture & Flow Diagram

When a client connects to DashDB, the connection follows a strict lifecycle. Below is the internal connection handling and operation flow:

```mermaid
sequenceDiagram
    participant Client
    participant TCP Listener (Tokio)
    participant Auth Handler
    participant Operation Router
    participant DashMap (Memory)

    Client->>TCP Listener (Tokio): Establish TCP Connection
    TCP Listener (Tokio)->>Auth Handler: Delegate Stream
    Client->>Auth Handler: Send [Opcode 255] + Token Length + Token
    
    alt Invalid Token
        Auth Handler-->>Client: [Opcode 2] (Error) & Close Stream
    else Valid Token
        Auth Handler-->>Client: [Opcode 1] (Success)
        
        loop Until Disconnect
            Client->>Operation Router: Send Command Opcode (1, 2, or 3) + Payload
            Operation Router->>DashMap: Execute Insert/Get/Remove
            DashMap-->>Operation Router: Result
            Operation Router-->>Client: Return Status & Payload
        end
    end