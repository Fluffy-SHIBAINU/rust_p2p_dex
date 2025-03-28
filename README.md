# rust_p2p_dex

# 🚀 Rust QUIC P2P DEX

A **high-performance** QUIC-based decentralized exchange (DEX) written in **Rust**. This project demonstrates **secure, low-latency** peer-to-peer (P2P) order transmission using QUIC.

<img width="834" alt="image" src="https://github.com/user-attachments/assets/b7e6b642-7172-47ec-9b7d-18348173b713" />




---

## 📌 Features

- **Secure QUIC connections** using Rustls
- **P2P order transmission** with QUIC streams
- **Automatic transport configuration** for optimized connection reliability
- **TLS certificate verification bypass for local testing**
- **Efficient QUIC keep-alive settings** for stable connections

---

## 🛠️ Installation & Setup

### ✅ Prerequisites

- Install **Rust & Cargo**:
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- Install dependencies:
  ```sh
  cargo install cargo-edit
  ```

### 📥 Clone the Repository

```sh
git clone https://github.com/yourusername/rust_p2p_dex.git
cd rust_p2p_dex
```

### 📦 Install Dependencies

```sh
cargo build
```

---

## 🚀 Running the Project

### 1️⃣ Start the QUIC Server

```sh
cargo run --bin rust_p2p_dex
```

✅ Expected output:

```
✅ QUIC 서버 실행 중: 127.0.0.1:5000
```

### 2️⃣ Start the Client & Send Orders

```sh
cargo run --bin client
```

✅ Expected output:

```
🚀 클라이언트: 주문 전송 준비 중...
✅ 서버 연결 성공: 127.0.0.1:5000
✅ 주문 전송 성공!
```

---

## ⚙️ Configuration

### 🔐 QUIC Transport Settings (Optimized for Stability)

The **`TransportConfig`** settings ensure:

- **Keep-Alive Enabled** (`keep_alive_interval(Some(Duration::from_secs(10)))`)
- **Extended Idle Timeout** (`max_idle_timeout(Some(Duration::from_secs(30)))`)

**Location:**

- **Server:** `build_server_config()` in `network.rs`
- **Client:** `build_client_config()` in `network.rs`

---

## ❌ Troubleshooting

### ❗ Error: `ApplicationClosed(ApplicationClose { error_code: 0, reason: b"" })`

✅ Solution: Increase the `max_idle_timeout` and enable `keep_alive_interval` in **both server & client.**

### ❗ Error: `Address already in use`

✅ Solution:

- Make sure no other process is using **port 5000**.
- Run:
  ```sh
  lsof -i :5000
  ```
- If needed, kill the process:
  ```sh
  kill -9 <PID>
  ```

### ❗ Error: `TransportError: invalid peer certificate`

✅ Solution: Add **`NoCertificateVerification`** in `build_client_config()`.

---

## 🛠️ Code Structure

```
📂 rust_p2p_dex
├── 📄 Cargo.toml         # Project dependencies
├── 📄 README.md          # Documentation
├── 📂 src
│   ├── 📄 main.rs        # Server main entry point
│   ├── 📄 client.rs      # Client entry point
│   ├── 📄 network.rs     # QUIC networking logic
```

---

## 🛡️ Security Considerations

- This project **disables TLS certificate validation** for local testing. In production, proper certificate handling is required.
- P2P communication should **implement authentication mechanisms** to prevent malicious activity.

---

## 📜 License

This project is licensed under the **MIT License**.

---

## 🙌 Contributing

Pull requests are welcome! For major changes, please open an issue first.

🚀 Happy Hacking!

