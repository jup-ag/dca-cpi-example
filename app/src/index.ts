import { AnchorProvider, BN, Program, web3 } from '@coral-xyz/anchor';
import { DCA, DCA_PROGRAM_ID_BY_CLUSTER, Network } from '@jup-ag/dca-sdk';
import {
  ComputeBudgetInstruction,
  ComputeBudgetProgram,
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  TransactionInstruction,
  sendAndConfirmTransaction,
} from '@solana/web3.js';
import { IDL } from '../../target/types/dca_integration';
import {
  NATIVE_MINT,
  createSyncNativeInstruction,
  getAssociatedTokenAddressSync,
} from '@solana/spl-token';
import { deriveEscrow, getOrCreateATAInstruction } from './helpers';
import { Decimal } from 'decimal.js';

const RPC = process.env.RPC || 'https://api.devnet.solana.com';
const connection = new Connection(RPC);

const programId = new PublicKey('5mrhiqFFXyfJMzAJc5vsEQ4cABRhfsP7MgSVgGQjfcrR');
const provider = new AnchorProvider(
  connection,
  {} as any,
  AnchorProvider.defaultOptions(),
);
const program = new Program(IDL, programId, provider);

const dcaClient = new DCA(connection, Network.MAINNET);

const user = Keypair.fromSecretKey(
  new Uint8Array(JSON.parse(process.env.USER_PRIVATE_KEY!)),
);

// USDC
// const inputMint = new PublicKey('EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v');
// const inputMintAmount = new Decimal('0.1').mul(1_000_000);

const bonkMint = new PublicKey('DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263');

const localMint = new PublicKey('HFpDk6RnhGCVs2fR4kXkxYuNdEiDRp6vRD2UPzhnz5Pc');

const inputMint = NATIVE_MINT;
const inputMintAmount = new Decimal('0.1').mul(LAMPORTS_PER_SOL);

// const inputMint = localMint;
// const inputMintAmount = new Decimal('1').mul(LAMPORTS_PER_SOL);

const outputMint = bonkMint;

async function setupDCA(
  userInTokenAccount: PublicKey,
  inputMint: PublicKey,
  outputMint: PublicKey,
  inAmount: string,
  inAmountPerCycle: string,
  cycleSecondsApart: string,
) {
  const uid = new BN(parseInt((Date.now() / 1000).toString()));
  const escrow = deriveEscrow(
    program.programId,
    user.publicKey,
    inputMint,
    outputMint,
    uid,
  );
  const dcaPubKey = await dcaClient.getDcaPubKey(
    escrow,
    inputMint,
    outputMint,
    uid,
  );

  const preInstructions: TransactionInstruction[] = [
    ComputeBudgetProgram.setComputeUnitLimit({
      units: 500_000,
    }),
  ];

  if (inputMint.equals(NATIVE_MINT)) {
    const { ataPubKey, ix } = await getOrCreateATAInstruction(
      connection,
      inputMint,
      user.publicKey,
    );
    const transferIx = web3.SystemProgram.transfer({
      fromPubkey: user.publicKey,
      lamports: new BN(inAmount).toNumber(),
      toPubkey: ataPubKey,
    });
    const syncNativeIX = createSyncNativeInstruction(ataPubKey);

    if (ix) {
      preInstructions.push(ix);
    }
    preInstructions.push(transferIx);
    preInstructions.push(syncNativeIX);
  }

  console.log({
    user: user.publicKey,
    userTokenAccount: userInTokenAccount,
    jupDcaProgram: DCA_PROGRAM_ID_BY_CLUSTER['mainnet-beta'],
    jupDca: dcaPubKey,
    jupDcaInAta: getAssociatedTokenAddressSync(inputMint, dcaPubKey, true),
    jupDcaOutAta: getAssociatedTokenAddressSync(outputMint, dcaPubKey, true),
    jupDcaEventAuthority: new PublicKey(
      'Cspp27eGUDMXxPEdhmEXFVRn6Lt1L7xJyALF3nmnWoBj',
    ),
    escrow,
    escrowInAta: getAssociatedTokenAddressSync(inputMint, escrow, true),
    escrowOutAta: getAssociatedTokenAddressSync(outputMint, escrow, true),
    inputMint: inputMint,
    outputMint: outputMint,
  });

  const tx = await program.methods
    .setupDca(
      uid,
      new BN(inAmount),
      new BN(inAmountPerCycle),
      new BN(cycleSecondsApart),
      null,
      null,
      null,
    )
    .accounts({
      user: user.publicKey,
      userTokenAccount: userInTokenAccount,
      jupDcaProgram: DCA_PROGRAM_ID_BY_CLUSTER['mainnet-beta'],
      jupDca: dcaPubKey,
      jupDcaInAta: getAssociatedTokenAddressSync(inputMint, dcaPubKey, true),
      jupDcaOutAta: getAssociatedTokenAddressSync(outputMint, dcaPubKey, true),
      jupDcaEventAuthority: new PublicKey(
        'Cspp27eGUDMXxPEdhmEXFVRn6Lt1L7xJyALF3nmnWoBj',
      ),
      escrow,
      escrowInAta: getAssociatedTokenAddressSync(inputMint, escrow, true),
      escrowOutAta: getAssociatedTokenAddressSync(outputMint, escrow, true),
      inputMint: inputMint,
      outputMint: outputMint,
    })
    .preInstructions(preInstructions)
    .transaction();

  try {
    const txHash = await sendAndConfirmTransaction(connection, tx, [user], {
      skipPreflight: false,
    });
    console.log('Created DCA Escrow: ', { txHash, dcaPubKey, escrow });
    return txHash;
  } catch (err) {
    console.log(err);
    throw err;
  }
}

async function close(
  dca: PublicKey,
  escrow: PublicKey,
  inputMint: PublicKey,
  outputMint: PublicKey,
) {
  const tx = await program.methods
    .close()
    .accounts({
      inputMint,
      outputMint,
      user: user.publicKey,
      userTokenAccount: getAssociatedTokenAddressSync(
        outputMint,
        user.publicKey,
        false,
      ),
      escrow,
      dca,
      escrowInAta: getAssociatedTokenAddressSync(inputMint, escrow, true),
      escrowOutAta: getAssociatedTokenAddressSync(outputMint, escrow, true),
    })
    .transaction();

  try {
    console.log('Closing');
    const txHash = await sendAndConfirmTransaction(connection, tx, [user], {
      skipPreflight: false,
    });
    console.log('Closed Escrow: ', { txHash });
    return txHash;
  } catch (err) {
    console.log(err);
    // throw err;
  }
}

async function findByUser(user: PublicKey) {
  return program.account.escrow.all([
    {
      memcmp: {
        offset: 8 + 8,
        bytes: user.toBase58(),
      },
    },
  ]);
}

// async function main() {
//   await setupDCA(
//     getAssociatedTokenAddressSync(inputMint, user.publicKey, true),
//     inputMint,
//     outputMint,
//     inputMintAmount.toFixed(),
//     inputMintAmount.div(new Decimal('2')).toString(),
//     '60',
//   );
// }

// async function main() {
//   const [escrow1] = await findByUser(
//     new PublicKey('7w32LzRsJrQiE7S3ZSdkz9TSFGey1XNsonPmdm9xDUch'),
//   );

//   const escrow = escrow1.publicKey;
//   const dca = escrow1.account.dca;
//   await close(
//     dca,
//     escrow,
//     escrow1.account.inputMint,
//     escrow1.account.outputMint,
//   );
// }

async function main() {
  const res = await findByUser(
    new PublicKey('7w32LzRsJrQiE7S3ZSdkz9TSFGey1XNsonPmdm9xDUch'),
  );

  const currentDcas = [];
  const completedDcas = [];

  for (const escrow of res) {
    console.log({ address: escrow.publicKey, info: escrow.account });

    currentDcas.push(...(await dcaClient.getCurrentByUser(escrow.publicKey)));
    completedDcas.push(...(await dcaClient.getClosedByUser(escrow.publicKey)));
  }
  console.log({ currentDcas, completedDcas });
}

main();
