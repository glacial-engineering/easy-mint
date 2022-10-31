# Easy Mint Program

## What is this

An onchan program which allows any project to permissionlessly create a "MintDefinition" - which acts somewhat like a "product" 
that can be purchased, and is delivered as an Metaplex SFT.

## Why?

Because a project I like wanted a way to mint a NFT whitelist token to people at Breakpoint using a simple QR code.

## How?

Derive a mint definition account from some (any) word, then call the create_mint_definition operation providing the required args.  
Anyone can now mint the token you defined by paying the price you defined.  

## A QR Code? No dapp?

Yeah, plan is to just use Solana Pay to build txs direct to their wallet when they scan the QR. Might add that code here, 
not sure, I'm not the one doing it.

## More detail

Can define:
- All properties of a metaplex mint (uri, sumbol, name, royalties, owner, etc) to define the token that will be minted upon purchase
- Owner, which has update price authority
- Price
- Price Mint
- Expiration date

## I don't understand

See the tests or hmu on Discord.
