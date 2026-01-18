# Submission Explainer

* **Goal:** I built this to solve the "Paper Hands" problem—preventing users from spending their savings early.
* **State Machine:** The contract uses a `Vault` struct stored in a PDA. It saves the `owner` (Pubkey) and `unlock_time` (i64).
* **Logic:** I used `Clock::get()?` to access the current block timestamp. The logic `if clock < unlock_time` acts as the guardrail.
* **Safety:** I implemented `require_keys_eq!` to ensure only the person who made the vault can open it.
* **Mutation:** The state only changes twice: once to initialize (write data) and once to withdraw (close account/erase data).
* **Design Choice:** I chose to use Anchor's `close = owner` feature in the Withdraw context. This is efficient because it refunds the storage rent to the user automatically.