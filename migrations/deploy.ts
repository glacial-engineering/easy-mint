// Migrations are an early feature. Currently, they're nothing more than this
// single deploy script that's invoked from the CLI, injecting a provider
// configured from the workspace's Anchor.toml.

import * as anchor from "@project-serum/anchor";
import { EasyMint } from "../target/types/easy_mint";
import * as token from "@solana/spl-token";
import * as mpl from "@metaplex-foundation/mpl-token-metadata"

module.exports = async function (provider) {
  // Configure client to use the provider.
  anchor.setProvider(provider);

  const program = anchor.workspace.EasyMint as anchor.Program<EasyMint>;

  let myMint = new anchor.web3.PublicKey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");
  let payerPaymentAta: anchor.web3.PublicKey;

  console.log("payer", provider.wallet.publicKey.toBase58());

  let memorableWord = "dana3";
  let mintDefinition = anchor.utils.publicKey.findProgramAddressSync([Buffer.from(memorableWord), provider.publicKey.toBuffer()], program.programId)[0];
  let mint = anchor.utils.publicKey.findProgramAddressSync([mintDefinition.toBuffer()], program.programId)[0];
  let metadata = anchor.utils.publicKey.findProgramAddressSync([Buffer.from("metadata"), mpl.PROGRAM_ID.toBuffer(), mint.toBuffer()], mpl.PROGRAM_ID)[0];

  // await provider.connection.confirmTransaction(
  //   await provider.connection.requestAirdrop(payer.publicKey, 100000000),
  //   "confirmed"
  // );

  // await delay(10000);

  // await provider.connection.confirmTransaction(
  //   await provider.connection.requestAirdrop(setupWallet.publicKey, 100000000),
  //   "confirmed"
  // );

  // await delay(10000);

  // await provider.connection.confirmTransaction(
  //   await provider.connection.requestAirdrop(vault_dude.publicKey, 100000000),
  //   "confirmed"
  // );

  // await delay(10000);

  //create mint to pay with
  // myMint = new anchor.web3.PublicKey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");

  // await delay(3000);

  // payerPaymentAta = await token.createAssociatedTokenAccount(provider.connection, payer, myMint, payer.publicKey);

  // await delay(3000);
  
  // await token.mintTo(provider.connection, setupWallet, myMint, payerPaymentAta, setupWallet, 1000);

  // await delay(3000);

  try {
    const tx = await program.methods
      .createMintDefinition(
        memorableWord,
        myMint,
        provider.publicKey,
        new anchor.BN(100),
        new anchor.BN(1698971600),
        "Dana Mint",
        "DM",
        "https://mtnphotobooth7afc22e0692f4205b848b9c01e02200e211757-dev.s3.amazonaws.com/public/zagZsEERU6ZhkEG7eoRJeJgeKKtaQ9BU5W55MDestoR.json",
        500
      )
      .accounts({
        owner: provider.publicKey,
        mintDefinition: mintDefinition,
        mint: mint,
        mintMetadataAccount: metadata,
        metadataProgram: mpl.PROGRAM_ID,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  } catch (e) {
    console.log(JSON.stringify(e, null, 2));
    throw e;
  }

  // await delay(3000);
  
  // try {
  //   const tx = await program.methods
  //     .pleaseMintToken()
  //     .accounts({
  //       payer: provider.publicKey,
  //       mintDefinition: mintDefinition,
  //       payWithMint: myMint,
  //       payFromTokenAcct: payerPaymentAta,
  //       mintDefinitionOwner: provider.publicKey,
  //       paymentMintDefinitionOwnerTokenAcct: await token.getAssociatedTokenAddress(myMint, provider.publicKey),
  //       recipientWallet: provider.publicKey,
  //       mint: mint,
  //       deliveryTokenAcct: await token.getAssociatedTokenAddress(mint, provider.publicKey),
  //     })
  //     .rpc();
  //   console.log("Your transaction signature", tx);
  // } catch (e) {
  //   console.log(JSON.stringify(e, null, 2));
  //   throw e;
  // }
};

function delay(t: number) {
  console.log("waiting", t);
  return new Promise((res, rej) => setTimeout(res, t));
}

