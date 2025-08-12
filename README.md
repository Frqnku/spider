# 🕷️ Spider

[![Crates.io](https://img.shields.io/crates/v/spider.svg)](https://crates.io/crates/spider)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-2021-orange.svg)](https://www.rust-lang.org/)

An **ultra-fast image scraper** built in **Rust**, leveraging **asynchronous programming** and **multi-threading** for unmatched performance.  
Spider crawls a website and downloads all its images, with an optional recursive mode to explore multiple depth levels.

---

## 🚀 Features

- **High-performance systems language: Rust**
- **Asynchronous** powered by [Tokio](https://tokio.rs/) to handle thousands of network requests in parallel
- **Safe multi-threading** with `Arc`, `Mutex`, `DashSet`, and semaphores to prevent race conditions
- **Parallel downloads** optimized with `reqwest` and `futures`
- **Duplicate handling** to avoid unnecessary downloads
- **Configurable recursive scraping** (depth limit)
- **Robust**: handles network errors and various image formats

---

## 📦 Installation

### Prerequisites
- [Rust](https://www.rust-lang.org/) installed (2021 edition or later)
- `cargo` available in your terminal

### Clone and build
```bash
git clone https://github.com/Frqnku/spider.git
cd spider
make build
```

The `spider` binary will be generated at the project root.

---

## 🛠 Usage

### Basic command
```bash
./spider <URL>
```

### Available options
| Option                  | Description                                                    | Default     |
|-------------------------|----------------------------------------------------------------|-------------|
| `-r`, `--recursive`     | Enable recursive scraping                                      | false       |
| `-l`, `--limit <N>`     | Max scraping depth (requires `--recursive`)                    | 5           |
| `-p`, `--path <PATH>`   | Directory to save images                                       | ./data      |
| `<URL>`                 | The URL of the page to scrape                                  | required    |

### Examples
```bash
# Scrape all images from a page
./spider https://example.com

# Recursively scrape up to 3 levels deep
./spider -r -l 3 https://example.com

# Specify a destination folder
./spider -p ./images https://example.com
```

---

## ⚙️ How it works

1. **CLI argument parsing** with `clap`
2. **Disk access checks** to ensure a valid and writable directory
3. **Asynchronous downloads** with `reqwest` and `futures`
4. **Image & link extraction** with a simple HTML parser
5. **Concurrency management** using:
   - `Arc` + `Mutex` to share state (`visited`)
   - `DashSet` for fast duplicate tracking
   - `Semaphore` to limit the number of concurrent tasks
6. **Safe file saving**: each image is saved with a unique name based on UUID and timestamp

---

## 📚 Tech Stack

- **Language**: Rust (2021 edition)
- **Networking**: [reqwest](https://docs.rs/reqwest/)
- **Async runtime**: [Tokio](https://tokio.rs/)
- **Concurrency**: `Arc`, `Mutex`, `Semaphore`, `DashSet`
- **URL parsing**: [url](https://docs.rs/url/)
- **Unique IDs**: [uuid](https://docs.rs/uuid/)
- **Timestamps**: [chrono](https://docs.rs/chrono/)
- **CLI**: [clap](https://docs.rs/clap/)

---

## 📜 License
This project is licensed under the MIT License.  
See the [LICENSE](LICENSE) file for details.

---

💡 *Spider is designed to maximize performance while remaining simple to use. Thanks to Rust and its safe async/multi-threaded architecture, it can handle a huge number of simultaneous downloads without overwhelming your system.*
