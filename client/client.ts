// Client Test Script for Commitment Vault
console.log("My address:", pg.wallet.publicKey.toString());

// 1. Define our Vault PDA (Program Derived Address)
// We need to find the address where the vault lives
const [vaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("vault"), pg.wallet.publicKey.toBuffer()],
  pg.program.programId
);

console.log("Vault Address found:", vaultPda.toString());

// 2. Initialize the Vault (Lock 1000 Lamports for 5 seconds)
try {
  const txHash = await pg.program.methods
    .initializeVault(new BN(5), new BN(1000)) // 5 seconds, 1000 lamports
    .accounts({
      vault: vaultPda,
      owner: pg.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();
  console.log("✅ Vault Initialized! Funds locked for 5 seconds.");
  console.log("Transaction:", txHash);
} catch (err) {
  console.log("Note: If this failed, you might have already initialized the vault. Try changing the wallet or redeploying.");
  console.error(err);
}

// 3. Try to Withdraw IMMEDIATELY (Should Fail)
console.log("🕵️  Attempting to withdraw early...");
try {
  await pg.program.methods
    .withdraw()
    .accounts({
      vault: vaultPda,
      owner: pg.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();
  console.log("❌ ERROR: Withdrawal succeeded early? That shouldn't happen!");
} catch (err) {
  // We EXPECT an error here, so this is actually good news!
  console.log("✅ SUCCESS! The contract blocked the early withdrawal.");
  console.log("   Reason:", err.error.errorMessage);
}

// 4. Wait for the time lock (6 seconds)
console.log("⏳ Waiting 6 seconds for the lock to expire...");
await new Promise((resolve) => setTimeout(resolve, 6000));

// 5. Withdraw AGAIN (Should Succeed)
console.log("🔓 Time is up! Attempting withdrawal now...");
try {
  const txHash = await pg.program.methods
    .withdraw()
    .accounts({
      vault: vaultPda,
      owner: pg.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();
  console.log("✅ SUCCESS! Funds withdrawn.");
  console.log("Transaction:", txHash);
} catch (err) {
  console.error("❌ Withdrawal failed:", err);
}