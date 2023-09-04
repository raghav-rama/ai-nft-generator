import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  PublicKey,
  Connection,
  Commitment,
  Transaction,
  sendAndConfirmTransaction,
  sendAndConfirmRawTransaction,
} from "@solana/web3.js";
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
  FunctionAccountInitParams,
  FunctionAccount,
  attestationTypes,
  AttestationQueueAccount,
  AttestationProgramStateAccount,
  BootstrappedAttestationQueue,
  SwitchboardWallet,
  getSwitchboardAttestationProgramId,
  NativeMint,
} from "@switchboard-xyz/solana.js";
import { parseRawMrEnclave } from "@switchboard-xyz/common";
import { DataV2 } from "@metaplex-foundation/mpl-token-metadata";

describe("ai-nft-generator", () => {
  const unixTimestamp = () => Math.floor(Date.now() / 1000);
  const MRENCLAVE = parseRawMrEnclave(
    "0x665aff2cfbdee0fcb1031ee788338da07af81a56172f7ca5042a47c985e8b272"
  );
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AiNftGenerator as Program<AiNftGenerator>;

  const payer = anchor.web3.Keypair.fromSecretKey(new Uint8Array(wallet));

  const commitment: Commitment = "confirmed";
  const connection = new Connection(
    // "https://api.devnet.solana.com",
    "http://localhost:8899",
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
  let functionVerifyIxn: anchor.web3.TransactionInstruction;
  let functionAccountData: attestationTypes.FunctionAccountData;
  let functionAccountG: FunctionAccount;
  let switchboardG: BootstrappedAttestationQueue;
  let requestAccountKeypairG: anchor.web3.Keypair;
  before(async () => {
    try {
      const switchboardProgram = await SwitchboardProgram.load(
        // "devnet",
        "localnet",
        connection,
        payer
      );
      await AttestationProgramStateAccount.getOrCreate(switchboardProgram);
      const switchboard: BootstrappedAttestationQueue =
        await AttestationQueueAccount.bootstrapNewQueue(switchboardProgram);
      const [wallet] = await SwitchboardWallet.create(
        switchboard.program,
        switchboard.attestationQueue.publicKey,
        payer.publicKey,
        "MySharedWallet",
        16
      );
      switchboardG = switchboard;
      const functionAccountInitParams: FunctionAccountInitParams = {
        name: "test function",
        metadata: "test metadata",
        container: "860x9/openai-request",
        version: "latest",
        containerRegistry: "dockerhub",
        mrEnclave: MRENCLAVE,
        authority: payer.publicKey,
        attestationQueue: switchboard.attestationQueue.account,
      };
      const [functionAccount, txId2] =
        await switchboard.attestationQueue.account.createFunction(
          functionAccountInitParams,
          wallet
        );
      functionAccountG = functionAccount;
      console.log("functionAccount", functionAccount.publicKey.toBase58());
      console.log("txId2", txId2);
      let functionTriggerParams: FunctionTriggerParams = {
        authority: payer,
      };
      const txId3 = await functionAccount.trigger(functionTriggerParams);
      console.log("txId3", txId3);
      // const [functionAccount, functionAccountData] = await FunctionAccount.load(
      //   switchboardProgram,
      //   "BvvrSwT1KFwXf8v4E7ZA1L7jdFTYzt1WcqBV8FfDtTJx"
      // );
      /*
      txId = await functionAccount.trigger(functionTriggerParams);
      console.log("txId", txId);*/
      functionAccountData = await functionAccount.loadData();
      await AttestationProgramStateAccount.getOrCreate(switchboardProgram);
      // const switchboard = await AttestationQueueAccount.bootstrapNewQueue(
      //   switchboardProgram
      // );

      const functionRequestAccountInitParams: FunctionRequestAccountInitParams =
        {
          functionAccount,
          containerParams: Buffer.from("cats in space"),
          authority: payer.publicKey,
        };
      const [functionRequestAccount, txId4] =
        await FunctionRequestAccount.create(
          switchboardProgram,
          functionRequestAccountInitParams
        );
      console.log(
        "functionRequestAccount",
        functionRequestAccount.publicKey.toBase58()
      );
      console.log("txId4", txId4);
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
  it("sets function", async () => {
    try {
      const tx = await program.methods
        .setFunction({})
        .accounts({
          function: functionAccountG.publicKey,
          program: programPda,
          authority: payer.publicKey,
        })
        .signers([payer])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }
  });
  it("sets the user prompt", async () => {
    try {
      const requestAccountKeypair = anchor.web3.Keypair.generate();
      requestAccountKeypairG = requestAccountKeypair;
      console.log(
        `requestAccountKeypair`,
        requestAccountKeypair.publicKey.toBase58()
      );
      const requestAccount = new FunctionRequestAccount(
        switchboardG.program,
        requestAccountKeypair.publicKey
      );
      const functionState = await functionAccountG.loadData();

      const tx = await program.methods
        .setUserPrompt({
          prompt: "cats in space",
        })
        .accounts({
          program: programPda,
          switchboard: switchboardG.program.attestationProgramId,
          state: switchboardG.program.attestationProgramState.publicKey,
          attestationQueue: switchboardG.attestationQueue.publicKey,
          function: functionAccountG.publicKey,
          requestAccount: requestAccountKeypair.publicKey,
          requestAccountEscrow: anchor.utils.token.associatedAddress({
            mint: NativeMint.address,
            owner: requestAccountKeypair.publicKey,
          }),
          mint: NativeMint.address,
          systemProgram: anchor.web3.SystemProgram.programId,
          payer: payer.publicKey,
        })
        .signers([requestAccountKeypair, payer])
        .rpc();
      console.log("Your transaction signature", tx);
      const requestState = await requestAccount.loadData();
      console.log("requestState", requestState.containerParams);
    } catch (error) {
      console.log(error);
    }
  });
  it("triggers the function", async () => {
    try {
      const tx = await program.methods
        .triggerFunction({})
        .accounts({
          function: functionAccountG.publicKey,
          requestAccount: requestAccountKeypairG.publicKey,
          attestationQueue: switchboardG.attestationQueue.publicKey,
          authority: payer.publicKey,
          atestationProgram: getSwitchboardAttestationProgramId("devnet"),
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([requestAccountKeypairG, payer])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }
  });
  it("generates the nft", async () => {
    try {
      const enclaveSigner = anchor.web3.Keypair.generate();
      console.log(`enclaveSigner`, enclaveSigner.publicKey.toBase58());
      const rewardAddress =
        await switchboardG.program.mint.getOrCreateAssociatedUser(
          payer.publicKey
        );
      functionVerifyIxn = attestationTypes.functionVerify(
        switchboardG.program,
        {
          params: {
            observedTime: new anchor.BN(unixTimestamp()),
            nextAllowedTimestamp: new anchor.BN(unixTimestamp() + 100),
            isFailure: false,
            mrEnclave: Array.from(MRENCLAVE),
          },
        },
        {
          function: functionAccountG.publicKey,
          functionEnclaveSigner: enclaveSigner.publicKey,
          verifier: switchboardG.verifier.publicKey,
          verifierSigner: switchboardG.verifier.signer.publicKey,
          attestationQueue: switchboardG.attestationQueue.publicKey,
          escrowWallet: functionAccountData.escrowWallet,
          escrowTokenWallet: functionAccountData.escrowTokenWallet,
          receiver: rewardAddress,
          verifierPermission: switchboardG.verifier.permissionAccount.publicKey,
          tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        }
      );
      const preTransaction = new Transaction().add(functionVerifyIxn);
      preTransaction.recentBlockhash = (
        await connection.getLatestBlockhash(commitment)
      ).blockhash;
      const txId = await sendAndConfirmTransaction(
        connection,
        preTransaction,
        [enclaveSigner, switchboardG.verifier.signer],
        { skipPreflight: true }
      ).then(async () => {
        console.log("pre txId", txId);
        const tx = await program.methods
          .mintAiNft({
            nft: {
              name: [1, 2, 3],
              symbol: [1, 2, 3],
              description: [1, 2, 3],
              image: [1, 2, 3],
              animationUrl: [1, 2, 3],
              externalUrl: [1, 2, 3],
            },
          })
          .accounts({
            function: functionAccountG.publicKey,
            oracle: oraclePda,
            enclaveSigner: enclaveSigner.publicKey,
            request: requestAccountKeypairG.publicKey,
          })
          .signers([enclaveSigner])
          .rpc();
        console.log("Your transaction signature", tx);
      });
    } catch (error) {
      console.log(error);
    }
  });
});
