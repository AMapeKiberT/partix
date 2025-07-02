<div align="center">

# PARTIX

🚀 Easy disk utility built with Rust for POSIX.




[![Rust](https://img.shields.io/badge/Rust-🦀-orange?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Performance](https://img.shields.io/badge/⚡️-blazing_fast-blue?style=for-the-badge)]()
[![License: AGPL-3.0](https://img.shields.io/badge/license-AGPL--3.0-blue?style=for-the-badge)](https://codeberg.org/titago/partix/src/branch/main/LICENSE)
[![cargo nightly](https://img.shields.io/badge/cargo-1.90.0--nightly-orange?style=for-the-badge&logo=cargo&logoColor=white)](https://rust-lang.github.io/rustup/installation/index.html)

</div>

---

## 📋 Table of Contents

1. [Features](#features)  
2. [Status & Roadmap](#status--roadmap)  
3. [Installation](#installation)  
4. [Usage Examples](#usage-examples)  
6. [Contributing](#contributing)  
7. [License](#license)

---

## ✨ Features <a id="features"></a>

- 🔹 **Intuitive CLI** — context‑aware help and suggestions  
- ⚡ **Blazing performance** — near‑instant operations  
- 🎨 **Clean output** — colorized, structured, and easy to parse  
- 🛠 **Comprehensive disk management** — list, delete partitions  
- 🔒 **Safe by design** — written in Rust, memory‑safe  
- 🌐 **Cross‑platform** — tested on Linux and Android

---

## 📊 Status & Roadmap <a id="status--roadmap"></a>

| Feature                                      | Status | Notes                                |
|----------------------------------------------|:------:|--------------------------------------|
|  List partitions                             |   ✓    |  Full verbose output                 |
|  Delete partition                            |   ✓    |  Removing, with auto-write           |
|  Create partition                            |   ✗    |  Add soon                            |
|  Support FS (ext4, ntfs, etc.)               |   ✗    |  Add filesystems soon                |
|  Show detailed info                          |   ✗    |  UUID, flags, mount status soon      |
|  Resize existing partition                   |   ✗    |  🔜 Planning                         |
|  CLI Interface                               |   ✓    |  CLI interface as in parted          |
|  TUI Interface                               |   ✗    |  TUI interface planned               |
|  GUI frontend                                |   ✗    |  Planning                            |

---

## 🚀 Installation <a id="installation"></a>
<!--
### ⚙️ From crates.io

```fish
cargo install partix
```
-->
### 🏗️ Building from source

```fish
git clone https://codeberg.org/titago/partix.git
```
```fish
cd partix
```
```fish
cargo build --release
```


---

## 🤝 Contributing <a id="contributing"></a>

We welcome contributions! Please:

    ⭐️ Star the repo

    🍴 Fork & create a branch (feature/xyz)

    📝 Add tests & documentation

    🔀 Open a Pull Request

---

## 📄 License <a id="license"></a>

This project is licensed under the AGPL-3.0 License.
See LICENSE for details.
