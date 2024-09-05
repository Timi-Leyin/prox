# Prox - A Caching Proxy Server (Rust)

Prox is a lightweight caching proxy server built with Rust that forwards requests to an origin server, caches the responses, and serves cached responses when available. It enhances performance by reducing redundant requests and improving response times.

## Features
- Forward requests to an origin server.
- Cache responses and serve from cache on repeated requests.
- Include headers indicating whether the response is from cache (`HIT`) or origin server (`MISS`).
- Simple command-line interface for starting the proxy server or clearing the cache.

## Installation
Ensure you have [Rust installed](https://www.rust-lang.org/tools/install) on your system.

Clone the repository and build the project:

```bash
git clone https://github.com/yourusername/prox.git
cd prox
cargo build --release
```
