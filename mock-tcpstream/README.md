# Mock TcpStream

This is a mocked `TcpStream` that implements many of the same traits as both
`std::net::TcpStream`, `tokio::net::TcpStream`, and `async_std::net::TcpStream`
so that, if generics are used with type bounds, the chief export,
`MockTcpStream`, can be used in lieu of real TCP stream types.

The primary use case of this is for testing purposes. With this library, you
don't have to create real TCP sockets in tests: you can just use
`MockTcpStream`, as long as the code used expects a generic type that implements
the following traits:

- `std::io::Read`
- `std::io::Write`
- `tokio::io::AsyncRead`
- `tokio::io::AsyncReadExt`
- `tokio::io::AsyncWrite`
- `tokio::io::AsyncWriteExt`
- `futures::io::AsyncRead`
- `futures::io::AsyncWrite`
- `std::net::ToSocketAddrs`
- `Debug`
- `Clone`
- `Default`
- `PartialEq`
- `Unpin`

## Installation

You should only add this to `dev-dependencies` of your `Cargo.toml`. You SHOULD
NOT include this in your `dependencies`.

Example:

```toml
[dev-dependencies]
mock-tcpstream-tokio = "1"
```

## Usage

When you call either the synchronous or asynchronous (Tokio) `write()`, instead
of data being written to the network, the bytes supplied are appended to
`MockTcpStream.transmitted_data`. When you call either the synchronous or
asynchronous (Tokio) `read()`, bytes are copied and removed from the front of
`MockTcpStream.received_data`.

To populate `received_data` with data (to simulate the receipt of data over the
network), call `receive()`. To clear the `transmitted_data` buffer, call
`transmit()`, which will also return the contents of this buffer; this simulates
the transmission of data over the network.

## Performance

This library was not designed to be efficient at all. The
point is to mock a `TcpStream` for testing purposes. Slow-performing code is a
known and accepted shortcoming of this library, so if you need extreme
performance in your tests, this might not be the library for you.

That said, unless you have thousands of tests, or if your tests are dealing
with gigabytes of data, you probably won't even notice any difference in
performance at all, since we're talking about a scale of a few milliseconds in
most cases.

## Error Handling

This library **DOES NOT** handle errors. `.unwrap()` is used liberally,
primarily for `std::sync::Mutex` unlocking, since this library is just for
testing purposes.

## Examples

Unfortunately, since both `std` and `tokio` define `read()` and `write()`
methods, you'll have to use the fully-qualified syntax to call these methods to
distinguish which one you are using.

### Synchronous Write

```rust
let mut stream = MockTcpStream::new();
std::io::Write::write(&mut stream, &[ 1, 2, 3, 4, 5 ]).unwrap();
{ // Scoped so the mutex lock drops.
    let tx_data = stream.transmitted_data.lock().unwrap();
    assert_eq!(tx_data.len(), 5);
}
```

### Asynchronous Write

```rust
let mut stream = MockTcpStream::new();
tokio::io::AsyncWriteExt::write(&mut stream, &[ 1, 2, 3, 4, 5 ]).await.unwrap();
{ // Scoped so the mutex lock drops.
    let tx_data = stream.transmitted_data.lock().unwrap();
    assert_eq!(tx_data.len(), 5);
}
```

### Synchronous Read

```rust
  let input = [ 1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 22, 33, 44 ];
  let mut stream = MockTcpStream::new();
  stream.receive(&input).unwrap();
  { // Scoped so the mutex lock drops.
      let rx_data = stream.received_data.lock().unwrap();
      assert_eq!(input.len(), rx_data.len());
  }
  { // Scoped so the mutex lock drops.
      let mut readbuf = [0; 7];
      let bytes_read = std::io::Read::read(&mut stream, &mut readbuf).unwrap();
      assert_eq!(bytes_read, 7);
      let rx_data = stream.received_data.lock().unwrap();
      assert_eq!(rx_data.len(), input.len() - 7);
  }
```

### Asynchronous Read

```rust
let input = [ 1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 22, 33, 44 ];
let mut stream = MockTcpStream::new();
stream.receive(&input).unwrap();
{ // Scoped so the mutex lock drops.
    let rx_data = stream.received_data.lock().unwrap();
    assert_eq!(input.len(), rx_data.len());
}
{ // Scoped so the mutex lock drops.
    let mut readbuf = [0; 7];
    let bytes_read = tokio::io::AsyncReadExt::read(&mut stream, &mut readbuf).await.unwrap();
    assert_eq!(bytes_read, 7);
    let rx_data = stream.received_data.lock().unwrap();
    assert_eq!(rx_data.len(), input.len() - 7);
}
```

## Location

This repository is a temporary home for this library. This will eventually be
moved.
