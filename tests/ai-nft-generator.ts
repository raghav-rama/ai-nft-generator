import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, Connection, Commitment } from "@solana/web3.js";
import { AiNftGenerator } from "../target/types/ai_nft_generator";
import wallet from "../wallet.json";
import {
  FunctionRequestSetConfigParams,
  FunctionRequestTriggerParams,
  FunctionRequestAccountInitParams,
  FunctionTriggerParams,
  FunctionRequestAccount,
  SendTransactionObjectOptions,
  SwitchboardProgram,
  FunctionAccount,
  attestationTypes,
  AttestationQueueAccount,
  AttestationProgramStateAccount,
} from "@switchboard-xyz/solana.js";
import { parseRawMrEnclave } from "@switchboard-xyz/common";

describe("ai-nft-generator", () => {
  const unixTimestamp = () => Math.floor(Date.now() / 1000);
  const MRENCLAVE = parseRawMrEnclave(
    "0x363b16f752945498c0fafef9f14915141ffdd35729eb39d8dd330161552703dc"
  );
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AiNftGenerator as Program<AiNftGenerator>;

  const payer = anchor.web3.Keypair.fromSecretKey(new Uint8Array(wallet));

  const commitment: Commitment = "confirmed";
  const connection = new Connection(
    "https://api.devnet.solana.com",
    commitment
  );

  interface InitializeParams {}
  let initializeParams: InitializeParams = {};

  const [programPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("ai-nft-generator")],
    program.programId
  );
  const [oraclePda] = PublicKey.findProgramAddressSync(
    [Buffer.from("ai-nft-generator-oracle")],
    program.programId
  );
  let txId;
  before(async () => {
    try {
      const switchboardProgram = await SwitchboardProgram.load(
        "devnet",
        connection,
        payer
      );
      const [functionAccount, functionAccountData] = await FunctionAccount.load(
        switchboardProgram,
        "BvvrSwT1KFwXf8v4E7ZA1L7jdFTYzt1WcqBV8FfDtTJx"
      );
      /*let functionTriggerParams: FunctionTriggerParams = {
        authority: payer,
      };
      txId = await functionAccount.trigger(functionTriggerParams);
      console.log("txId", txId);*/
      /*await AttestationProgramStateAccount.getOrCreate(switchboardProgram);
      const switchboard = await AttestationQueueAccount.bootstrapNewQueue(
        switchboardProgram
      );
      const securedSigner = anchor.web3.Keypair.generate();
      const rewardAddress =
        await switchboard.program.mint.getOrCreateAssociatedUser(
          payer.publicKey
        );
      const functionVerifyIxn = attestationTypes.functionVerify(
        switchboard.program,
        {
          params: {
            observedTime: new anchor.BN(unixTimestamp()),
            nextAllowedTimestamp: new anchor.BN(unixTimestamp() + 100),
            isFailure: false,
            mrEnclave: Array.from(MRENCLAVE),
          },
        },
        {
          function: functionAccount.publicKey,
          functionEnclaveSigner: securedSigner.publicKey,
          verifier: switchboard.verifier.publicKey,
          verifierSigner: switchboard.verifier.signer.publicKey,
          attestationQueue: switchboard.attestationQueue.publicKey,
          escrowWallet: functionAccountData.escrowWallet,
          escrowTokenWallet: functionAccountData.escrowTokenWallet,
          receiver: rewardAddress,
          verifierPermission: switchboard.verifier.permissionAccount.publicKey,
          tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        }
      );*/
      // const functionRequestAccountInitParams: FunctionRequestAccountInitParams =
      //   {
      //     functionAccount,
      //     containerParams: Buffer.from("prompt"),
      //     authority: payer.publicKey,
      //   };
      // const [functionRequestAccount, txId] =
      //   await FunctionRequestAccount.create(
      //     switchboardProgram,
      //     functionRequestAccountInitParams
      //   );
      /*const [functionRequestAccount, functionRequestAccountData1] =
        await FunctionRequestAccount.load(
          switchboardProgram,
          "BsNUF2vKEXqwFf49DyXDNd75UrYL69gGKxUYGq3uuddf"
        );
      // console.log("functionRequestAccountData1", functionRequestAccountData1);
      const functionRequestAccountData =
        await functionRequestAccount.loadData();
      // console.log("functionRequestAccountData", functionRequestAccountData);
      let functionRequestTriggerParams: FunctionRequestTriggerParams = {
        authority: payer,
      };
      txId = await functionRequestAccount.trigger(functionRequestTriggerParams);
      console.log("txId", txId);
      const functionRequestSetConfigParams: FunctionRequestSetConfigParams = {
        appendContainerParams: false,
        containerParams: Buffer.from("prompt3"),
        authority: payer,
      };
      txId = await functionRequestAccount.setConfig(
        functionRequestSetConfigParams
      );
      console.log("txId", txId);*/
    } catch (err) {
      console.log(err);
    }
  });

  it("Is initialized!", async () => {
    // Add your test here.
    try {
      console.log("programPda", programPda.toBase58());
      const tx = await program.methods
        .initialize({})
        .accounts({
          program: programPda,
          oracle: oraclePda,
          payer: payer.publicKey,
          authority: payer.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([payer])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.error(error);
    }
  });
});
