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

  //let myMint = new anchor.web3.PublicKey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");
  let myMint = new anchor.web3.PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
  
  //let payTo = new anchor.web3.PublicKey("Dana2JeMV2W74btnsquyoWh3CFTe57FjYt3B3w4GgYUm");
  let payTo = new anchor.web3.PublicKey("AQGGa2C2JHoVKprpHVhxt1vmff6jgfwLgy9A6yzsFkbV");
  console.log("payer", provider.wallet.publicKey.toBase58());

  let memorableWord = "hl-og-usdc-1";
  let mintDefinition = anchor.utils.publicKey.findProgramAddressSync([Buffer.from(memorableWord), provider.publicKey.toBuffer()], program.programId)[0];
  let mint = anchor.utils.publicKey.findProgramAddressSync([mintDefinition.toBuffer()], program.programId)[0];
  let metadata = anchor.utils.publicKey.findProgramAddressSync([Buffer.from("metadata"), mpl.PROGRAM_ID.toBuffer(), mint.toBuffer()], mpl.PROGRAM_ID)[0];

  console.log("creating mint definition", mintDefinition.toBase58());

  try {
    const tx = await program.methods
      .createMintDefinition(
        memorableWord,
        payTo,
        myMint,
        new anchor.BN(225000000),
        new anchor.BN(1669852800),
        "PussyDAO OG List Token",
        "PD-HL-1",
        "https://4ufbz4hahjk5ueuu4dbvkeb6ts4p5ddkxa553l37346wgv6pvrwa.arweave.net/5Qoc8OA6VdoSlODDVRA-nLj-jGq4O92vf989Y1fPrGw?ext=json",
        //"https://gdybaqxreesa6kvg73kqgv5mb3ab5wmdjdndc24dk6argdzce33a.arweave.net/MPAQQvEhJA8qpv7VA1esDsAe2YNI2jFrg1eBEw8iJvY?ext=json",
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

