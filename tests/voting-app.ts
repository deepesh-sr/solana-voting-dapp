import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VotingApp } from "../target/types/voting_app";
import { assert } from "chai";

describe("voting-app", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.VotingApp as Program<VotingApp>;

  it("Initializes a poll", async () => {
    // Generate a new Keypair for the poll account
    const poll = anchor.web3.Keypair.generate();
    const pollId = new anchor.BN(1);
    const description = "Best programming language?";
    const pollStart = new anchor.BN(Date.now() / 1000);
    const pollEnd = new anchor.BN(Date.now() / 1000 + 86400); // +1 day

    // Derive the PDA for the poll account
    const [pollPDA, _] = await anchor.web3.PublicKey.findProgramAddress(
      [pollId.toArrayLike(Buffer, "le", 8)],
      program.programId
    );

    // Call the initialize_poll instruction
    await program.methods
      .initializePoll(pollId, description, pollStart, pollEnd)
      .accounts({
        signer: provider.wallet.publicKey,
      })
      .signers([])
      .rpc();

    // Fetch the poll account and assert its fields
    const pollAccount = await program.account.poll.fetch(pollPDA);
    assert.equal(pollAccount.pollId.toNumber(), pollId.toNumber());
    assert.equal(pollAccount.description, description);
    assert.equal(pollAccount.pollStart.toNumber(), pollStart.toNumber());
    assert.equal(pollAccount.pollEnd.toNumber(), pollEnd.toNumber());
    assert.equal(pollAccount.candidateAmount.toNumber(), 0);
  });

});
