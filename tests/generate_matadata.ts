// // eslint-disable-next-line node/no-unpublished-import
// import type { AiNftGenerator } from "../target/types/ai_nft_generator";

// import type { Program } from "@coral-xyz/anchor";
// import * as anchor from "@coral-xyz/anchor";
// import { parseRawMrEnclave, sleep } from "@switchboard-xyz/common";
// import type { FunctionAccount, MrEnclave } from "@switchboard-xyz/solana.js";
// import { SwitchboardWallet } from "@switchboard-xyz/solana.js";
// import {
//   AttestationProgramStateAccount,
//   AttestationQueueAccount,
//   attestationTypes,
//   type BootstrappedAttestationQueue,
//   SwitchboardProgram,
//   types,
// } from "@switchboard-xyz/solana.js";

// const unixTimestamp = () => Math.floor(Date.now() / 1000);

// // vv1gTnfuUiroqgJHS4xsRASsRQqqixCv1su85VWvcP9

// const MRENCLAVE = parseRawMrEnclave(
//     "0xec4e9cb9c6e78c0a008e35ea92e3a9478d9bfd14e15bbf88c93935f0951c1d98"
//     );
// const emptyEnclave: number[] = new Array(32).fill(0);

// describe("ai-nft-generator", () => {
//   // Configure the client to use the local cluster.
//   anchor.setProvider(anchor.AnchorProvider.env());

//   const program = anchor.workspace.AiNftGenerator as Program<AiNftGenerator>;

//   console.log(`ProgramID: ${program.programId}`);

//   const payer = (program.provider as anchor.AnchorProvider).publicKey;

//   const programStatePubkey = anchor.web3.PublicKey.findProgramAddressSync(
//     [Buffer.from("ai-nft-generator")],
//     program.programId
//   )[0];

//   const oraclePubkey = anchor.web3.PublicKey.findProgramAddressSync(
//     [Buffer.from("ai-nft-generator-oracle")],
//     program.programId
//   )[0];

//   let switchboard: BootstrappedAttestationQueue;
//   let wallet: SwitchboardWallet;
//   let functionAccount: FunctionAccount;

//   before(async () => {
//     const switchboardProgram = await SwitchboardProgram.fromProvider(
//       program.provider as anchor.AnchorProvider
//     );

//     await AttestationProgramStateAccount.getOrCreate(switchboardProgram);

//     switchboard = await AttestationQueueAccount.bootstrapNewQueue(
//       switchboardProgram
//     );

//     console.log(`programStatePubkey: ${programStatePubkey}`);

//     [wallet] = await SwitchboardWallet.create(
//       switchboard.program,
//       switchboard.attestationQueue.publicKey,
//       payer,
//       "MySharedWallet",
//       16
//     );

//     console.log(`wallet: ${wallet.publicKey}`);

//     [functionAccount] =
//       await switchboard.attestationQueue.account.createFunction(
//         {
//           name: "test function",
//           metadata: "this function handles XYZ for my protocol",
//           schedule: "15 * * * * *",
//           container: "switchboardlabs/basic-oracle-function",
//           version: "latest",
//           mrEnclave: MRENCLAVE,
//           authority: programStatePubkey,
//         },
//         wallet
//       );

//     console.log(`functionAccount: ${functionAccount.publicKey}`);
//   });

//   it("Is initialized!", async () => {
//     // Add your test here.
//     const tx = await program.methods
//       .initialize({})
//       .accounts({
//         program: programStatePubkey,
//         oracle: oraclePubkey,
//         authority: payer,
//         payer: payer,
//         // function: functionAccount.publicKey,
//       })
//       .rpc()
//       .catch((err) => {
//         console.error(err);
//         throw err;
//       });
//     console.log("Your transaction signature", tx);
//   });

//   // it("Adds an enclave measurement", async () => {
//   //   // Add your test here.
//   //   const tx = await program.methods
//   //     .setEnclaves({ mrEnclaves: [Array.from(MRENCLAVE)] })
//   //     .accounts({
//   //       program: programStatePubkey,
//   //       authority: payer,
//   //     })
//   //     .rpc()
//   //     .catch((err) => {
//   //       console.error(err);
//   //       throw err;
//   //     });
//   //   console.log("Your transaction signature", tx);
//   //   const programState = await program.account.myProgramState.fetch(
//   //     programStatePubkey
//   //   );
//   // });

//   it("generate metadata", async () => {
//     const securedSigner = anchor.web3.Keypair.generate();

//     const rewardAddress =
//       await switchboard.program.mint.getOrCreateAssociatedUser(payer);

//     const functionState = await functionAccount.loadData();

//     // TODO: generate function verify ixn
//     const functionVerifyIxn = attestationTypes.functionVerify(
//       switchboard.program,
//       {
//         params: {
//           observedTime: new anchor.BN(unixTimestamp()),
//           nextAllowedTimestamp: new anchor.BN(unixTimestamp() + 100),
//           isFailure: false,
//           mrEnclave: Array.from(MRENCLAVE),
//         },
//       },
//       {
//         function: functionAccount.publicKey,
//         functionEnclaveSigner: securedSigner.publicKey,
//         verifier: switchboard.verifier.publicKey,
//         verifierSigner: switchboard.verifier.signer.publicKey,
//         attestationQueue: switchboard.attestationQueue.publicKey,
//         escrowWallet: functionState.escrowWallet,
//         escrowTokenWallet: functionState.escrowTokenWallet,
//         receiver: rewardAddress,
//         verifierPermission: switchboard.verifier.permissionAccount.publicKey,
//         tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
//       }
//     );

//     // generate nft metadata
//     const tx = await program.methods
//       .mintNft({
//         {
//             "name": "My NFT",
//             "description": "My NFT Description",
//             "image": "https://gateway.pinata.cloud/ipfs/QmZJk9tQh8J5Z6E6iXhV6QZ4fY4G8mN2Z5mZ5Qy3V3XrZ1",
//             "external_url": "https://gateway.pinata.cloud/ipfs/QmZJk9tQh8J5Z6E6iXhV6QZ4fY4G8mN2Z5mZ5Qy3V3XrZ1",
//         }
//       })
//       .accounts({
//         oracle: oraclePubkey,
//         function: functionAccount.publicKey,
//         enclaveSigner: securedSigner.publicKey,
//       })
//       .preInstructions([functionVerifyIxn])
//       .signers([switchboard.verifier.signer, securedSigner])
//       .rpc({ skipPreflight: true });

//     console.log("Your transaction signature", tx);

//     await printLogs(switchboard.program.connection, tx ? tx : "");

//     await sleep(5000);

//     const oracleState = await program.account.myOracleState.fetch(oraclePubkey);

//     console.log(oracleState);

//   });
// });

// function normalizeDecimals(value: anchor.BN) {
//   return (value ?? new anchor.BN(0))
//     .div(new anchor.BN(10).pow(new anchor.BN(9)))
//     .toNumber();
// }
