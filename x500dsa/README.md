# X.500 DSA

## Advice

- From https://corrode.dev/blog/async/
  - Try to write it using threads and blocking I/O
    - It is faster: https://vorner.github.io/async-bench.html
  - Never expose async APIs in crates.
- `ureq` seems to be most promising for sync HTTP request needs
- I think you'll want to use `parking_lot::RwLock` for the `HashMap`s that
  contain knowledge, schema info, etc.
- This might be a desirable way to do logging: https://crates.io/crates/tracing
- I am going to do i18n in logs, somewhat against common advice
  - Still use short english strings for searchability
  - I am going to implement this using traits and objects.
- Use RusTLS instead of OpenSSL
-

## Proof-of-Concept To Do

- [ ] Read IDM frames from a socket
- [ ] IDM Protocol State Machine
- [ ] Configuration (YAML, TOML, or Rhai?)
- [ ] Server Struct
- [ ] Association Struct
- [ ] Request Struct
- [ ] Error Type
- [ ] Storage Interface Definition
- [ ] Bind Operation
- [ ] Add Entry Operation
- [ ] Search Operation
- [ ] Unbind Operation
- [ ] Logging
- [ ] Command-Line
- [ ] Minimal X.500 Schema
