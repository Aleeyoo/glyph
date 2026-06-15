# Async I/O Deferred to v0.2

## Why async is not in v0.1

### 1. Threading or Tokio integration required
Proper async I/O in a terminal-file-pager like `glyph` requires either:

- A background thread pool to perform reads, writes, and stat calls without blocking the UI event loop, or
- Integration with a runtime like Tokio, with the main loop driving async tasks alongside terminal input.

Either approach adds significant complexity. The current architecture is a single-threaded, synchronous event loop. Introducing an async runtime means either spawning a dedicated I/O thread (with the associated sync/async bridge and teardown) or reworking the event loop to be async-native. Both are non-trivial and better deferred until the core user-facing model is stable.

### 2. Simple sync file I/O is acceptable for v0.1

For a v0.1 terminal pager, the I/O profile is simple:

- Read a file into a buffer on open.
- Stat a file for metadata.
- Occasionally re-read a file (e.g. on SIGHUP or manual reload).

None of these operations are high-frequency or latency-sensitive at the scale of a single-user terminal tool. Blocking on a file read of a few megabytes is imperceptible next to terminal rendering latency. Introducing async before there is a demonstrated bottleneck would be premature optimization.

### 3. Plan for v0.2

Revisit async I/O in v0.2 when the following conditions arise:

- Support for streaming input (pipes, network sockets, dynamic content) where reads are unbounded and non-blocking is genuinely required.
- The event loop has been factored to support concurrent tasks (e.g. a poll-based model or a Tokio runtime).
- Performance profiling shows that file I/O is a measurable bottleneck.

Until then, `std::fs::read` and friends are the right tool.
