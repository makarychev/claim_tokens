import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ClaimTokens } from "../target/types/claim_tokens";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";

const HOLDING_PREFIX = "holding";

describe("claim_tokens", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const connection = provider.connection;

  const program = anchor.workspace.ClaimTokens as Program<ClaimTokens>;
  const wallet = provider.wallet as anchor.Wallet;
  const payer = wallet.payer;

  it("Is initialized!", async () => {
    // Add your test here.
    const decimals = 6;
    const maxClaimAmount = 100 * 10 ** decimals;
    const maxClaimCount = 3;

    const holdingMintPubkey = await createMint(
      connection,
      payer,
      wallet.publicKey,
      null,
      decimals
    );

    const ruleData = anchor.web3.Keypair.generate();
    const [holdingOwnerPubkey, holdingOwnerBump] =
      anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(HOLDING_PREFIX), holdingMintPubkey.toBuffer()],
        program.programId
      );

    const holdingTokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      payer,
      holdingMintPubkey,
      holdingOwnerPubkey,
      true
    );
    console.log(`holdingPubkey: ${holdingTokenAccount.address}`);

    const tx = await program.methods
      .initializeRule(
        new anchor.BN(maxClaimAmount),
        maxClaimCount,
        holdingOwnerBump
      )
      .accounts({
        rule: ruleData.publicKey,
        holdingAddress: holdingTokenAccount.address,
        holdingOwner: holdingOwnerPubkey,
        holdingMint: holdingMintPubkey,
        payer: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([ruleData])
      .rpc();

    console.log("Your transaction signature", tx);
  });
});
