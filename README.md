# 🎟️ Ticket Resale Smart Contract (Soroban - Stellar)

## 📌 Project Description

This project is a decentralized ticket resale platform built using Soroban smart contracts on the Stellar blockchain. It addresses common issues in traditional ticketing systems such as fraud, duplicate tickets, and lack of transparency.

By leveraging blockchain technology, this system ensures that ticket ownership is verifiable, transfers are secure, and resale is conducted in a trustless environment without intermediaries.

<img width="1920" height="1080" alt="ticket resale smart contract" src="https://github.com/user-attachments/assets/08ea78f1-db60-4db8-9aba-63c211535e67" />

---

## ⚙️ What It Does

The smart contract enables a complete ticket lifecycle on-chain:

* 🎫 **Ticket Creation** – Event organizers or owners can create tickets with unique IDs.
* 🔄 **Ticket Resale Listing** – Ticket holders can list their tickets for resale at a desired price.
* 🛒 **Ticket Purchase** – Buyers can securely purchase tickets from sellers.
* 🔍 **Ownership Verification** – Anyone can query and verify ticket ownership and status.

All operations are executed on-chain, ensuring immutability and transparency.

---

## ✨ Features

* 🔐 **Secure Ownership Control**
  Only the ticket owner can list or manage their ticket.

* 🔄 **Decentralized Resale Marketplace**
  Peer-to-peer ticket resale without relying on centralized platforms.

* ⚡ **Lightweight Soroban Contract**
  Optimized for performance with minimal storage overhead.

* 📊 **Transparent Data Access**
  Ticket details (owner, price, sale status) are publicly accessible.

* 🛡️ **Authorization Enforcement**
  Uses `require_auth()` to ensure only valid users perform actions.

---

## 🔗 Deployed Smart Contract

**Contract Address:**

```
CAPGCTQNJLGFIXSNN7U3ATXNY55ABC7VMC25OXUOIMAWJHUWZVKKEMQK
```

You can interact with this contract using the Stellar CLI or integrate it into a frontend dApp.

---

## 🛠️ Tech Stack

* **Blockchain:** Stellar
* **Smart Contracts:** Soroban (Rust)
* **SDK:** soroban-sdk

---

## 🚀 How to Use (CLI Example)

```bash
# Invoke create ticket
stellar contract invoke \
  --id CAPGCTQNJLGFIXSNN7U3ATXNY55ABC7VMC25OXUOIMAWJHUWZVKKEMQK \
  --fn create_ticket \
  --arg id=1 \
  --arg owner=<YOUR_ADDRESS> \
  --arg price=100
```

---

## 🔮 Future Improvements

* 💰 Payment integration (XLM / tokens)
* 🎫 NFT-based ticket standard
* 📅 Event metadata (date, venue, seat info)
* 🚫 Anti-scalping price limits
* 🌐 Frontend marketplace UI

---

## 📜 License

MIT License
