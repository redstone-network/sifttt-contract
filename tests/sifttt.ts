import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Sifttt } from "../target/types/sifttt";
import { expect } from "chai";

describe("sifttt", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Sifttt as Program<Sifttt>;

  it("Is initialized!", async () => {
    // Generate keypair for the new account
    const newAccount = anchor.web3.Keypair.generate();

    try {
      const tx = await program.methods
        .initialize()
        .accounts({
          account: newAccount.publicKey,
          user: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([newAccount])
        .rpc();

      // Fetch the account and verify it was initialized
      const account = await program.account.accountState.fetch(newAccount.publicKey);
      expect(account.borrowUtilization.toString()).to.equal("0");
      
      console.log("Transaction signature", tx);
    } catch (error) {
      console.error("Error:", error);
      throw error;
    }
  });
});
