import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AiNftGenerator } from "../target/types/ai_nft_generator";

describe("ai-nft-generator", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AiNftGenerator as Program<AiNftGenerator>;

  const payer = anchor.web3.Keypair.generate();

  interface InitializeParams {}
  let initializeParams: InitializeParams = {};

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize(undefined)
      .accounts({
        oracle: payer.publicKey,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
