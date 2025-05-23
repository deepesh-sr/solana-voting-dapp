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
    const pollId = new anchor.BN(1);
    const description = "Best programming language?";
    const pollStart = new anchor.BN(Date.now() / 1000);
    const pollEnd = new anchor.BN(Date.now() / 1000 + 86400); // +1 day

    // Derive the PDA for the poll account
    const [pollPDA,] = await anchor.web3.PublicKey.findProgramAddress(
      [pollId.toArrayLike(Buffer, "le", 8)],
      program.programId
    );

    // Call the initializepoll instruction
    await program.methods
      .initializePoll(pollId, description, pollStart, pollEnd)
      .accountsStrict({
        signer: provider.wallet.publicKey,
        poll: pollPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // Fetch the poll account and assert its fields
    const pollAccount = await program.account.poll.fetch(pollPDA);
    assert.equal(pollAccount.pollId.toNumber(), pollId.toNumber());
    assert.equal(pollAccount.description, description);
    assert.equal(pollAccount.pollStart.toNumber(), pollStart.toNumber());
    assert.equal(pollAccount.pollEnd.toNumber(), pollEnd.toNumber());
    assert.equal(pollAccount.candidateAmount.toNumber(), 0);
  });

  it("Initialize Candidate", async() =>{
    const pollId = new anchor.BN(1);
    const candidate_name = "Rust";

    const [candidatePDA,] = await anchor.web3.PublicKey.findProgramAddress(
      [pollId.toArrayLike(Buffer, "le", 8),Buffer.from(candidate_name)],
      program.programId
    )
    // Derive the PDA for the poll account
    const [pollPDA,] = await anchor.web3.PublicKey.findProgramAddress(
      [pollId.toArrayLike(Buffer, "le", 8)],
      program.programId
    )

    //call the initialize candidate instruction
    await program.methods
      .initializeCandidate(pollId,candidate_name)
      .accountsStrict({
        signer: provider.wallet.publicKey,
        poll : pollPDA,
        candidateAccount : candidatePDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      }).rpc();

      // Fetch the candidate account and assert its fields
      const candidateAccount = await program.account.candidate.fetch(candidatePDA);
      assert.equal(candidateAccount.candidateName, candidate_name);
      assert.equal(candidateAccount.candidateVote.toNumber(), 0);
  })
});