# Inception-Core

! Work in progress

---

> Modular Memory Server for Cognitive AI Systems

**Inception** is a local memory server (cortex) designed for use in agent-based and cognitive systems such as [`synthamind`](https://github.com/nbursa/synthamind) and [`sentience`](https://github.com/nbursa/sentience). It provides structured memory, a query engine, STM/LTM hierarchy, and REST/gRPC/WebSocket interfaces.

---

## Key Features

- Short-Term and Long-Term Memory model
- Query engine (simple DSL: `subject=ball AND object=red`)
- Pluggable storage backends: in-memory, file-based, RocksDB
- REST API with optional gRPC and WebSocket support
- Easy integration into other systems (via `rpc_client`)
- Fully local, no external dependencies

---

## Getting Started

### 1. Clone and run

```bash
git clone https://github.com/nbursa/inception
cd inception
cargo run
```

### 2. Configure `.env`

```env
API_ADDRESS=127.0.0.1:8080
MEMORY_BACKEND=inmemory
```

---

## Usage Examples

### Store a memory token

```bash
curl -X POST http://localhost:8080/memory/store \
  -H "Content-Type: application/json" \
  -d '{"subject":"ball","relation":"is","object":"red","tags":["toy"]}'
```

### Recall memory

```bash
curl -X POST http://localhost:8080/memory/recall \
  -H "Content-Type: application/json" \
  -d '{"query":"subject=ball AND object=red"}'
```

---

## Integrating from Other Projects

```rust
let client = InceptionClient::new("http://localhost:8080");

client.store_token(&MemoryToken {
    subject: "door".into(),
    relation: "is".into(),
    object: "closed".into(),
    tags: vec!["sensor".into()],
}).await?;
```

---

## License

MIT © Nenad Bursać  
Inspired by biological memory and the architecture of higher cognitive systems.
