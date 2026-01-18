# 💎 Commitment Vault (Diamond Hands)

A Solana smart contract that enforces savings discipline by locking funds until a specific timestamp. Built with Rust and Anchor for Build & Learn Week.

## 🎯 Project Summary
The "Commitment Vault" is an on-chain savings account. Users can deposit funds, but the contract prevents withdrawal until a specific `unlock_time` has passed. It uses the blockchain's native clock to enforce "Diamond Hands"—preventing impulsive selling or spending.

## ⚙️ How It Works
1.  **Initialize:** The user calls `initialize_vault` with a lock duration (in seconds) and an amount.
2.  **Lock:** The program transfers SOL from the user to a PDA (Program Derived Address) that the program controls.
3.  **Wait:** If the user tries to `withdraw` before the time is up, the contract returns a custom error (`VaultLocked`).
4.  **Withdraw:** Once `Clock::get().unix_timestamp > unlock_time`, the user can withdraw. The contract closes the account and refunds all rent + funds.

## 🛠️ Tech Stack
* **Language:** Rust
* **Framework:** Anchor
* **Network:** Solana Devnet

## 🔒 Security Features
* **Signer Checks:** Only the original owner can withdraw.
* **Time-Lock Logic:** Uses on-chain `Clock` sysvar for validation.
* **PDA State:** Funds are held in a program-controlled address, not a personal wallet.