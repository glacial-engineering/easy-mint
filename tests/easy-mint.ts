import * as anchor from "@project-serum/anchor";
import { EasyMint } from "../target/types/easy_mint";
import * as token from "@solana/spl-token";
import * as mpl from "@metaplex-foundation/mpl-token-metadata"

describe("easy-mint", () => {
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.EasyMint as anchor.Program<EasyMint>;

  let setupWallet = anchor.web3.Keypair.generate();
  let payer = anchor.web3.Keypair.generate();
  let vault_dude = anchor.web3.Keypair.generate();
  let myMint: anchor.web3.PublicKey;
  let payerPaymentAta: anchor.web3.PublicKey;

  let memorableWord = "dana";
  let mintDefinition = anchor.utils.publicKey.findProgramAddressSync([Buffer.from(memorableWord), vault_dude.publicKey.toBuffer()], program.programId)[0];
  let mint = anchor.utils.publicKey.findProgramAddressSync([mintDefinition.toBuffer()], program.programId)[0];
  let metadata = anchor.utils.publicKey.findProgramAddressSync([Buffer.from("metadata"), mpl.PROGRAM_ID.toBuffer(), mint.toBuffer()], mpl.PROGRAM_ID)[0];

  it("Set up!", async () => {
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(setupWallet.publicKey, 10000000000),
      "confirmed"
    );

    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(payer.publicKey, 10000000000),
      "confirmed"
    );

    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(vault_dude.publicKey, 10000000000),
      "confirmed"
    );

    //create mint to pay with
    myMint = await token.createMint(provider.connection, setupWallet, setupWallet.publicKey, setupWallet.publicKey, 9);
    payerPaymentAta = await token.createAssociatedTokenAccount(provider.connection, payer, myMint, payer.publicKey);
    await token.mintTo(provider.connection, setupWallet, myMint, payerPaymentAta, setupWallet, 1000);
  });

  it("Configured!", async () => {

    const expDate = Math.floor(Date.now() / 1000) + 10;
    console.log("expiry is", expDate);

    try {
      const tx = await program.methods
        .createMintDefinition(
          memorableWord,
          vault_dude.publicKey,
          myMint,
          new anchor.BN(100),
          new anchor.BN(expDate),
          "Dana Mint",
          "DM",
          "metadataurl",
          500
        )
        .accounts({
          owner: vault_dude.publicKey,
          mintDefinition: mintDefinition,
          mint: mint,
          mintMetadataAccount: metadata,
          metadataProgram: mpl.PROGRAM_ID,
        })
        .signers([vault_dude])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (e) {
      console.log(JSON.stringify(e, null, 2));
      throw e;
    }
  });

  it("Minted!", async () => {
    try {
      const tx = await program.methods
        .pleaseMintToken()
        .accounts({
          payer: payer.publicKey,
          mintDefinition: mintDefinition,
          payWithMint: myMint,
          payToAccount: vault_dude.publicKey,
          payFromTokenAcct: payerPaymentAta,
          mintDefinitionOwner: vault_dude.publicKey,
          payToTokenAcct: await token.getAssociatedTokenAddress(myMint, vault_dude.publicKey),
          recipientWallet: payer.publicKey,
          mint: mint,
          deliveryTokenAcct: await token.getAssociatedTokenAddress(mint, payer.publicKey),
        })
        .signers([payer])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (e) {
      console.log(JSON.stringify(e, null, 2));
      throw e;
    }
  });
});

function delay(t) {
  return new Promise((res, rej) => setTimeout(res, t));
}