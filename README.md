<div align="center">

# memviz

**A lightweight terminal UI for visualizing Linux memory usage in real time.**

[![Rust](https://img.shields.io/badge/Rust-2021-orange?logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Linux-lightgrey?logo=linux&logoColor=white)](#requirements)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](#contributing)

[Features](#features) · [Installation](#installation) · [Usage](#usage) · [How It Works](#how-it-works) · [Contributing](#contributing)

</div>

---

## Overview

`memviz` is a fast, dependency-light memory visualizer that runs entirely in your terminal. Built in Rust with [`ratatui`](https://github.com/ratatui/ratatui), it reads memory statistics directly from the Linux `/proc` filesystem and renders them as an interactive text-based dashboard, with no GUI, daemon, or background service required.

It is designed for developers, sysadmins, and anyone who wants a quick, readable picture of where system memory is going, including over SSH.

<!-- Add a screenshot or terminal recording here:
![memviz demo](docs/demo.gif)
-->

## Features

- **Terminal-native UI** — interactive dashboard rendered with `ratatui` and `crossterm`
- **Live data from `/proc`** — reads system memory information directly through the `procfs` crate, with no external tools to parse
- **Lightweight and fast** — a single compiled Rust binary with a small dependency footprint
- **SSH-friendly** — works anywhere you have a terminal, including remote servers and headless machines
- **Open source** — released under the MIT license

## Requirements

| Requirement | Details |
|-------------|---------|
| **OS** | Linux (relies on the `/proc` filesystem) |
| **Rust** | Stable toolchain, edition 2021 ([install Rust](https://www.rust-lang.org/tools/install)) |
| **Terminal** | Any ANSI-capable terminal emulator |

> **Note:** Because `memviz` depends on `procfs`, it is supported on Linux only. macOS and Windows are not supported.

## Installation

### From source

```bash
# Clone the repository
git clone https://github.com/faridhafizh/memviz.git
cd memviz

# Build an optimized release binary
cargo build --release

# The binary is now at ./target/release/memviz
```

### Install with Cargo

```bash
cargo install --git https://github.com/faridhafizh/memviz
```

This places the `memviz` binary in `~/.cargo/bin`, which should already be on your `PATH` if Rust is installed via `rustup`.

## Usage

Run directly with Cargo during development:

```bash
cargo run --release
```

Or launch the compiled binary:

```bash
./target/release/memviz
```

Press `Ctrl+C` to exit at any time.

## How It Works

`memviz` follows a simple pipeline:

1. **Collect** — memory statistics are read from the Linux `/proc` filesystem via [`procfs`](https://crates.io/crates/procfs).
2. **Process** — raw values are converted into human-readable figures and ratios.
3. **Render** — [`ratatui`](https://crates.io/crates/ratatui) draws the interface, while [`crossterm`](https://crates.io/crates/crossterm) handles the terminal backend and keyboard input.

### Tech stack

| Crate | Version | Purpose |
|-------|---------|---------|
| [`ratatui`](https://crates.io/crates/ratatui) | 0.29 | Terminal UI widgets and layout |
| [`crossterm`](https://crates.io/crates/crossterm) | 0.28 | Cross-platform terminal control and input events |
| [`procfs`](https://crates.io/crates/procfs) | 0.17 | Access to Linux `/proc` system information |

## Project Structure

```
memviz/
├── src/             # Application source code
├── Cargo.toml       # Package manifest and dependencies
├── Cargo.lock       # Locked dependency versions
├── LICENSE          # MIT license
└── README.md
```

## Development

```bash
# Run in debug mode
cargo run

# Check for errors without building
cargo check

# Run the test suite
cargo test

# Format and lint
cargo fmt
cargo clippy -- -D warnings
```

## Roadmap

- [ ] Per-process memory breakdown
- [ ] Swap usage visualization
- [ ] Configurable refresh interval
- [ ] Historical usage graphs
- [ ] Prebuilt release binaries via GitHub Actions

Have an idea? [Open an issue](https://github.com/faridhafizh/memviz/issues) and let's discuss it.

## Contributing

Contributions are welcome and appreciated.

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature`
3. Commit your changes: `git commit -m "Add your feature"`
4. Push to your branch: `git push origin feature/your-feature`
5. Open a Pull Request

Please run `cargo fmt` and `cargo clippy` before submitting, and keep pull requests focused on a single change.

## License

Distributed under the MIT License. See [`LICENSE`](LICENSE) for details.

---

<div align="center">

If you find `memviz` useful, consider giving it a ⭐ on GitHub.

</div>
