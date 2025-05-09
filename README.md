# 🚀 FastFetch

**FastFetch** is a blazing-fast, asynchronous command-line utility written in Rust that downloads multiple files concurrently from the internet.

It uses modern async Rust tools like [`tokio`](https://crates.io/crates/tokio) and [`reqwest`](https://crates.io/crates/reqwest) to efficiently fetch and save content from provided URLs.

---

## 📦 Features

- 🔄 Concurrent downloading of multiple files
- ✨ Async file saving using `tokio::fs`
- 🧠 Modular, clean, and idiomatic Rust code
- 📝 Automatic filename detection from URLs
- 📁 (Optional) Easy to extend with features like retry logic and progress bars

---

## 🛠️ Technologies Used

- [Rust](https://www.rust-lang.org/)
- [Tokio](https://tokio.rs/) – for async runtime and task management
- [Reqwest](https://docs.rs/reqwest) – for HTTP client capabilities
- [tokio::fs](https://docs.rs/tokio/latest/tokio/fs/index.html) – for asynchronous file writing

---

## 📂 Project Structure


---

## 🚀 How to Run

### 1. Clone the Repository

```bash
git clone https://github.com/dimka90/FastFetch.git
cd FastFetch
```

## 2. Add Your URLs
Run the project with one or more URLs as command-line arguments:

```
cargo run -- "https://example.com/file1.txt" "https://example.com/file2.txt"
```
🧪 Example
```
cargo run -- "https://www.w3.org/TR/PNG/iso_8859-1.txt"
```
This will download the text file and save it as iso_8859-1.txt.

🔧 Future Improvements
 Add retry logic for failed downloads

 Show download progress using indicatif

 Support downloading binary files

 Parallel file writing with backpressure

