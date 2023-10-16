import { AnchorProvider, BorshAccountsCoder, Program } from '@coral-xyz/anchor';
import {
  Connection,
  Keypair,
  PublicKey,
  sendAndConfirmTransaction,
  SystemProgram,
} from '@solana/web3.js';
import { IDL } from '../../target/types/dca_integration';
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync,
} from '@solana/spl-token';
import { bs58 } from '@coral-xyz/anchor/dist/cjs/utils/bytes';
import * as dotenv from 'dotenv';
import NodeWallet from '@coral-xyz/anchor/dist/cjs/nodewallet';

dotenv.config();

// const escrowProgramId = new PublicKey(
//   '5mrhiqFFXyfJMzAJc5vsEQ4cABRhfsP7MgSVgGQjfcrR',
// );
const escrowProgramId = new PublicKey(
  'BoDCAjKTzVkunw5xx5r3EPWqe3uyNABJJjSRCJNoRmZa',
);

const BONK_MINT = new PublicKey('DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263');

//console.log(new PublicKey('auuSyC1KwkNwZv7BKxXXrWTjym3jW6YpFqeovpZ7i5h').toBuffer().toJSON().data)
// const [vaultSigner, bump] = PublicKey.findProgramAddressSync([Buffer.from('vault')], escrowProgramId )
// console.log(vaultSigner.toString(), bump)
// console.log(vaultSigner.toBuffer().toJSON().data)

const RPC = process.env.RPC || 'https://api.devnet.solana.com';
const connection = new Connection(RPC);

const admin = Keypair.fromSecretKey(
  new Uint8Array(JSON.parse(process.env.ADMIN_PRIVATE_KEY!)),
);
const signer = admin;

const provider = new AnchorProvider(
  connection,
  new NodeWallet(admin),
  AnchorProvider.defaultOptions(),
);
const program = new Program(IDL, escrowProgramId, provider);

async function createVault() {
  connection.onLogs(
    escrowProgramId,
    (logs, _ctx) => {
      console.log(logs);
    },
    'processed',
  );
  //   console.log('creating vault', {
  //     signer: signer.publicKey,
  //     vaultSigner: vaultSigner,
  //     vault: getAssociatedTokenAddressSync(BONK_MINT, vaultSigner, true),
  //     mint: BONK_MINT,
  //   })

  //   const hash = await program.methods.createVault().accountsStrict({
  //     signer: signer.publicKey,
  //     vaultSigner: vaultSigner,
  //     vault: getAssociatedTokenAddressSync(BONK_MINT, vaultSigner, true),
  //     mint: BONK_MINT,
  //     systemProgram: SystemProgram.programId,
  //     tokenProgram: TOKEN_PROGRAM_ID,
  //     associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //   }).signers([signer]).rpc()

  // console.log(hash)
}

async function getAccountsToAirdrop() {
  const offset = 169; // 8 + 8 + 32 * 4 + 8 * 3 + 1

  const coder = new BorshAccountsCoder(IDL);

  // const layouts = IDL.accounts.map((acc) => {
  //   return [acc.name as string, IdlCoder.typeDefLayout(acc, idl.types)];
  // });

  // console.log(IDL.accounts.map(a => a.name))
  // console.log(IDL.accounts.map(a => BorshAccountsCoder.accountDiscriminator(a.name).toJSON().data))

  let accts = await connection.getProgramAccounts(escrowProgramId);

  for (let acct of accts) {
    let parsed = { error: 'true' };
    try {
      parsed = coder.decodeAny(acct.account.data);
    } catch (error) {
      continue;
    }

    console.log({
      owner: acct.account.owner,
      data: acct.account.data.toJSON().data.length,
      head: acct.account.data.toJSON().data.slice(0, 8),
      parsed,
      // @ts-ignore
      input: parsed?.inputAmount?.toString(),
    });
  }
  console.log('total', accts.length);

  // const accountsYetToAirdrop = await program.account.escrow.all(
  // //   [
  // //   {
  // //     memcmp: {
  // //       offset,
  // //       bytes: bs58.encode([0]),
  // //     },
  // //   },
  // // ]

  // );

  // const accountsToAirdrop = accountsYetToAirdrop.filter((escrow) => {
  //   return escrow.account.completed;
  // });

  // return accountsToAirdrop;
}

async function airdrop() {
  const accountsToAirdrop = await getAccountsToAirdrop();
  console.log({ accountsToAirdrop });

  // await executeAirdrops(accountsToAirdrop)
}

async function executeAirdrops(accountsToAirdrop: any[]) {
  try {
    const res = await Promise.all(
      accountsToAirdrop.map(async (escrow) => {
        const tx = await program.methods
          .airdrop()
          .accounts({
            admin: admin.publicKey,
            user: escrow.account.user,
            escrow: escrow.publicKey,
            outputMint: escrow.account.outputMint,
            adminTokenAccount: getAssociatedTokenAddressSync(
              escrow.account.outputMint,
              admin.publicKey,
              false,
            ),
            userTokenAccount: getAssociatedTokenAddressSync(
              escrow.account.outputMint,
              escrow.account.user,
              false,
            ),
          })
          .transaction();

        return sendAndConfirmTransaction(connection, tx, [admin], {
          skipPreflight: false,
          commitment: 'confirmed',
        });
      }),
    );
    console.log({ res });
  } catch (err) {
    throw err;
  }
}

// async function main() {
//   while (true) {
//     try {
//       await airdrop();
//       await new Promise((resolve) => setTimeout(resolve, 5000));
//     } catch (err) {
//       // add error handling if needed
//       throw err;
//     }
//   }
// }

async function main() {
  await createVault();
}

main();
