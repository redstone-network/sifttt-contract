import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Sifttt } from "../target/types/sifttt";
import { expect } from "chai";

describe("sifttt", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Sifttt as Program<Sifttt>;
  let accountKeypair: anchor.web3.Keypair;

  beforeEach(async () => {
    accountKeypair = anchor.web3.Keypair.generate();
  });

  it("Is initialized with correct values", async () => {
    // Initialize account
    await program.methods
      .initialize()
      .accounts({
        account: accountKeypair.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([accountKeypair])
      .rpc();

    // Verify initialization
    const account = await program.account.accountState.fetch(accountKeypair.publicKey);
    expect(account.healthFactor.toString()).to.equal("100");
    expect(account.triggerHealthFactor.toString()).to.equal("0");
    expect(account.targetHealthFactor.toString()).to.equal("0");
    expect(account.automationEnabled).to.be.false;
  });

  it("Can set automation parameters", async () => {
    // Initialize first
    await program.methods
      .initialize()
      .accounts({
        account: accountKeypair.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([accountKeypair])
      .rpc();

    // Set automation
    await program.methods
      .setAutomation(new anchor.BN(60), new anchor.BN(80))
      .accounts({
        account: accountKeypair.publicKey,
        user: provider.wallet.publicKey,
      })
      .rpc();

    // Verify automation settings
    const account = await program.account.accountState.fetch(accountKeypair.publicKey);
    expect(account.triggerHealthFactor.toString()).to.equal("60");
    expect(account.targetHealthFactor.toString()).to.equal("80");
    expect(account.automationEnabled).to.be.true;
  });

  it("Handles borrow and repay operations", async () => {
    // Initialize
    await program.methods
      .initialize()
      .accounts({
        account: accountKeypair.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([accountKeypair])
      .rpc();

    // Borrow
    await program.methods
      .borrow()
      .accounts({
        account: accountKeypair.publicKey,
        user: provider.wallet.publicKey,
      })
      .rpc();

    let account = await program.account.accountState.fetch(accountKeypair.publicKey);
    expect(account.healthFactor.toString()).to.equal("90"); // 100 - 10

    // Repay
    await program.methods
      .repay()
      .accounts({
        account: accountKeypair.publicKey,
        user: provider.wallet.publicKey,
      })
      .rpc();

    account = await program.account.accountState.fetch(accountKeypair.publicKey);
    expect(account.healthFactor.toString()).to.equal("95"); // 90 + 5
  });

  it("Triggers auto-repay when conditions are met", async () => {
    // Initialize
    await program.methods
      .initialize()
      .accounts({
        account: accountKeypair.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([accountKeypair])
      .rpc();

    // Set automation
    await program.methods
      .setAutomation(new anchor.BN(70), new anchor.BN(90))
      .accounts({
        account: accountKeypair.publicKey,
        user: provider.wallet.publicKey,
      })
      .rpc();

    // Borrow multiple times to reduce health factor
    for (let i = 0; i < 4; i++) {
      await program.methods
        .borrow()
        .accounts({
          account: accountKeypair.publicKey,
          user: provider.wallet.publicKey,
        })
        .rpc();
    }

    // Auto-repay
    await program.methods
      .autoRepay()
      .accounts({
        account: accountKeypair.publicKey,
        user: provider.wallet.publicKey,
      })
      .rpc();

    // Verify health factor was restored
    const account = await program.account.accountState.fetch(accountKeypair.publicKey);
    expect(account.healthFactor.toString()).to.equal("90");
  });
});
