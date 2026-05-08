# staxup

**staxup** is a cross‑platform binary updater and version manager designed for the
Stax ecosystem — but flexible enough for any standalone CLI tool.

It provides a consistent, reliable, and secure way to distribute, update, and
rollback self‑contained binaries across Linux, macOS, and Windows.  
Tools like **StaxPing**, **StaxSpec**, and future Stax utilities all use staxup as
their update engine.

staxup is written in Rust, licensed under Apache 2.0, and built to be simple,
predictable, and ecosystem‑friendly.

---

## ✨ Features

- **Cross‑platform binary updates**  
  Linux, macOS, Windows — no installers required.

- **Manifest‑driven**  
  Tools define their metadata in a simple TOML file (e.g. `staxping.toml`).

- **Rollback support**  
  Every update keeps the previous version for instant recovery.

- **Channels**  
  Stable, beta, nightly — fully supported.

- **Checksums & integrity verification**  
  SHA‑256 validation for all downloaded binaries.

- **Self‑contained folder structure**  
  Tools install into:
  ```
  ~/.local/share/stax/<tool>/
  ```

- **Library + CLI**  
  Use staxup as a Rust crate *or* as a standalone command.

- **Registry‑optional**  
  Works with GitHub Releases, custom URLs, or a Stax registry.

- **Ecosystem‑ready**  
  Designed for multi‑tool suites like Stax, but generic enough for external use.

---

## 📦 Installation

staxup is distributed as a standalone binary and as a Rust crate.

### As a CLI tool
```
staxup update <tool>
staxup check <tool>
staxup rollback <tool>
```

### As a Rust library
```rust
use stax_up::Updater;

fn main() -> anyhow::Result<()> {
    Updater::new("staxping")
        .with_manifest_path("staxping.toml")
        .with_remote_manifest("https://example.com/staxping.toml")
        .check_and_apply()?;

    Ok(())
}
```

---

## 🗂 Folder Structure

staxup installs tools into a predictable, isolated directory:

```
~/.local/share/stax/<tool>/
    latest/       # active version
    old/          # rollback version
    <tool>.toml   # installed manifest
```

This structure is shared across all Stax tools.

---

## 📝 Tool Manifest Format

Each tool ships with a simple TOML manifest:

```toml
version = "2.0.0"
channel = "stable"
min_updater = "1.0.0"

[downloads]
linux_amd64 = "https://example.com/tool-linux-amd64"
windows_x64 = "https://example.com/tool-windows-x64.exe"

[checksums]
linux_amd64 = "sha256-..."
windows_x64 = "sha256-..."

[meta]
name = "StaxPing"
description = "Network diagnostics tool"
license = "Apache-2.0"
```

staxup reads this file to determine how to update the tool.

---

## 🔧 CLI Usage

```
staxup check <tool>     # Show current version and info
staxup update <tool>    # Update to latest version
staxup rollback <tool>  # Rollback to previous version
```

---

## 🏗 Building

```bash
cargo build --release
```

The binary will be in `target/release/staxup`.

---

## 🧩 Why staxup?

Most updaters fall into one of these categories:

- language‑specific package managers  
- GUI app updaters  
- OS‑level package managers  
- single‑tool self‑updaters  

staxup fills a different niche:

**A unified, cross‑tool updater for standalone CLI binaries.**

It is:

- language‑agnostic  
- ecosystem‑friendly  
- manifest‑driven  
- rollback‑capable  
- cross‑platform  
- simple to embed  

This makes it ideal for multi‑tool suites like Stax — and equally useful for
independent CLI tools that want a clean, reliable update story.

---

## 🛠 Roadmap

- [x] Initial crate + CLI scaffolding  
- [x] Manifest parser  
- [x] Version comparison  
- [x] Download + checksum verification  
- [x] Install + rollback logic  
- [ ] Self‑update support  
- [ ] Global registry integration  
- [ ] StaxDash GUI integration  

---

## 📄 License

Licensed under the **Apache License, Version 2.0**.  
See the `LICENSE` file for details.
