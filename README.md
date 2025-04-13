# 🛡️ MidTrust

**MidTrust** is a zero-knowledge peer-to-peer escrow service that enables two users — a sender and a receiver — to complete a transaction securely and **trustlessly**, without relying on third parties or intermediaries.

---

## 🔍 What It Does

MidTrust allows the **sender** to create a deal using a secret (like a passphrase). The system generates a **hash** from that secret and stores it with the deal (including sender, receiver, and amount). Later, the **receiver** must submit the correct secret to complete the transaction.

This utilizes a **zero-knowledge proof** — where the receiver proves they know a secret **without revealing it**, and the system can verify it **without ever storing or seeing the secret itself**.

---

## 🔁 How It Works

1. The sender inputs their wallet, the receiver’s wallet, a secret, and the deal amount.
2. The system hashes the secret using SHA256 and creates a deal with `Pending` status.
3. The receiver, who has the original secret, submits it to claim the deal.
4. If the hashed version of the submitted secret matches the stored hash, the deal is marked as `Released`.

✅ No secrets are stored.  
✅ The match between hashes acts as a **zero-knowledge-style proof**.

---

## ⚙️ Built With

- 🦀 Rust (`axum` backend)
- 🖥️ Yew (Rust/WASM frontend)
- 🔐 SHA256 hashing (`sha2` crate)
- 🟦 TypeScript – Strongly typed JavaScript for scalable and maintainable frontend + backend development
- 🌘 Midnight SDK – Privacy-preserving smart contract platform ideal for secure dApps and confidential P2P logic
- 🐳 Docker – for isolated, reproducible development environments
- 🧠 Zero-knowledge-style verification logic

---

## 🌐 Use Cases

- 🔒 Secure freelance transactions
- 🏠 Rental or deposit agreements
- 🎮 In-game or digital item exchanges
- 🗳️ DAO micro-contracts or bounty proofs

---
