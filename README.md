<!-- Copyright Andrik Seeger 2022-->

# 🦀 Bitcoin Mining in Rust

This project demonstrates a highly efficient, low-level implementation of a **Bitcoin miner in pure Rust**, connecting to mining pools using the **Stratum V2 protocol** and performing SHA-256-based mining operations via CPU.

---

## ⚙️ How It Works

* A TCP connection is established to a mining pool using **Stratum V2**.
* For testing, included credentials can be used to connect to a public pool.
* Upon connection, the miner receives data which is translated into mining jobs.
* Each job is processed by iterating over **Nonce** and **Extranonce** values.
* A custom **double SHA-256 hashing algorithm** is applied to search for a valid solution.
* Once a solution is found, it is submitted back to the mining pool via the established connection.

This miner was designed with **efficiency in mind**, using optimized Rust code to achieve high processing speed per CPU cycle — with no extensive mining libraries or dependencies.

---

## 🚀 Setup & Usage

### Requirements

* Rust (stable toolchain)
* Internet access for pool communication

### Build Instructions

```bash
git clone https://github.com/AndrikSeeger/Rust_Bitcoin_Mining.git
cd Rust_Bitcoin_Mining
cargo build --release
```

### Run the Miner

```bash
./target/release/rust_bitcoin_miner <username> <password>
```

Replace `<username>` and `<password>` with your mining pool credentials.
For testing, you can use the sample data included in the `Pool_Test_Access` directory.

---

## 📌 Additional Notes

* This project is a **proof of concept**, not a production miner.
* It runs **entirely on CPU**, meaning it cannot compete with modern GPU or ASIC rigs in terms of hashrate.
* Despite that, the code is highly optimized and efficient within its design goals.

---

## ❓ FAQ

**Can I use this miner to earn Bitcoin?**
Technically yes, but realistically no. With CPU-only mining, the hashrate is **less than 0.01%** of modern mining hardware. It's extremely unlikely to win a block, but great for experimentation and learning.

**Can I use the included login data?**
Yes, the credentials are provided for testing purposes only.

---

## 👥 Contributors

* Andrik Seeger
* Tom Schubert
