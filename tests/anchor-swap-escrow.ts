import { randomBytes } from 'node:crypto';
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorSwapEscrow } from "../target/types/anchor_swap_escrow";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { getAssociatedTokenAddressSync, TOKEN_2022_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { BN } from "bn.js";
import { confirmTransaction, createAccountsMintsAndTokenAccounts, makeKeypairs } from '@solana-developers/helpers';
import { assert } from 'chai';

const TOKEN_PROGRAM: typeof TOKEN_2022_PROGRAM_ID | typeof TOKEN_PROGRAM_ID = TOKEN_2022_PROGRAM_ID;

const SECONDS = 1000;

const ANCHOR_SLOW_TEST_THRESHOLD = 40 * SECONDS;

const getRandomBigNumber = (size = 8) => {
  return new BN(randomBytes(size));
};

describe("anchor-swap-escrow", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.anchorSwapEscrow as Program<AnchorSwapEscrow>;

  const user = (provider.wallet as anchor.Wallet).payer;
  const payer = user;
  
  const connection = provider.connection;

  const accounts: Record<string, PublicKey> = {
    tokenProgram: TOKEN_PROGRAM,
  };

  let alice: anchor.web3.Keypair;
  let bob: anchor.web3.Keypair;
  let mintA: anchor.web3.Keypair;
  let mintB: anchor.web3.Keypair;

  [alice, bob, mintA, mintB] = makeKeypairs(4);  
  
  const tokenAOfferedAmount = new BN(1_000_000);
  const tokenBWantedAmount = new BN(1_000_000);


  before("Creates Alice and Bob accounts, 2 token mints, and associated token accounts for both tokens for both users", async () => {
    const userMintsAndTokenAccounts = await createAccountsMintsAndTokenAccounts(
      [
        [
          1_000_000_000,
          0
        ],
        [
          0,
          1_000_000_000
        ],
      ],
      1 * LAMPORTS_PER_SOL,
      connection,
      payer
    );

    const users = userMintsAndTokenAccounts.users;
    alice = users[0];
    bob = users[1];

    const mints = userMintsAndTokenAccounts.mints;
    mintA = mints[0];
    mintB = mints[1];

    const tokenAccounts = userMintsAndTokenAccounts.tokenAccounts;

    const aliceTokenAccountA = tokenAccounts[0][0];
    const aliceTokenAccountB = tokenAccounts[0][1];

    const bobTokenAccountA = tokenAccounts[1][0];
    const bobTokenAccountB = tokenAccounts[1][1];

    accounts.maker = alice.publicKey;
    accounts.taker = bob.publicKey;
    accounts.mintA = mintA.publicKey;
    accounts.makerTokenAccountA = aliceTokenAccountA;
    accounts.takerTokenAccountA = bobTokenAccountA;
    accounts.mintB = mintB.publicKey;
    accounts.makerTokenAccountB = aliceTokenAccountB;
    accounts.takerTokenAccountB = bobTokenAccountB;
  });

  it("Puts tokens Alice offers into to vault, when alice makes an offer", async () => {
    const offerId = getRandomBigNumber();

    const offer = PublicKey.findProgramAddressSync(
      [Buffer.from('offer'), accounts.maker.toBuffer(), offerId.toArrayLike(Buffer, 'le', 8)],
      program.programId,
    )[0];

    const vault = getAssociatedTokenAddressSync(accounts.mintA, offer, true, TOKEN_PROGRAM);

    accounts.offer = offer;
    accounts.vault = vault;

    const transactionSignature = await program.methods
      .makeOffer(offerId, tokenAOfferedAmount, tokenBWantedAmount)
      .accounts({ ...accounts })
      .signers([alice])
      .rpc();

    await confirmTransaction(connection, transactionSignature);

    // Check our vault contains the tokens offered
    const vaultBalanceResponse = await connection.getTokenAccountBalance(vault);
    const vaultBalance = new BN(vaultBalanceResponse.value.amount);
    assert(vaultBalance.eq(tokenAOfferedAmount));
    
    // Check our Offer account contains the correct data
    const offerAccount = await program.account.offer.fetch(offer);

    assert(offerAccount.maker.equals(alice.publicKey));
    assert(offerAccount.mintA.equals(accounts.mintA));
    assert(offerAccount.mintB.equals(accounts.mintB));
    assert(offerAccount.wantedAmount.eq(tokenBWantedAmount));
  }).slow(ANCHOR_SLOW_TEST_THRESHOLD);

  it("Puts the tokens from the vault into Bob's account, and gives Alice Bob's tokens, when Bob takes an offer", async () => {
    const tx = await program.methods
      .takeOffer()
      .accounts({ ...accounts })
      .signers([bob])
      .rpc();

      await confirmTransaction(connection, tx);
  }).slow(ANCHOR_SLOW_TEST_THRESHOLD);
});