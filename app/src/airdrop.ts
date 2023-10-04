import { AnchorProvider, Program } from '@coral-xyz/anchor';
import {
  Connection,
  Keypair,
  PublicKey,
  sendAndConfirmTransaction,
} from '@solana/web3.js';
import { IDL } from '../../target/types/dca_integration';
import { getAssociatedTokenAddressSync } from '@solana/spl-token';
import { bs58 } from '@coral-xyz/anchor/dist/cjs/utils/bytes';

const RPC = process.env.RPC || 'https://api.devnet.solana.com';
const connection = new Connection(RPC);

const escrowProgramId = new PublicKey(
  'EXDCASuSBHrbJqf3tbap86YeWaGoEeCqBhGRbUSnoDqm',
);
const provider = new AnchorProvider(
  connection,
  {} as any,
  AnchorProvider.defaultOptions(),
);
const program = new Program(IDL, escrowProgramId, provider);

const admin = Keypair.fromSecretKey(
  new Uint8Array(JSON.parse(process.env.ADMIN_PRIVATE_KEY!)),
);

async function getAccountsToAirdrop() {
  const offset = 169; // 8 + 8 + 32 * 4 + 8 * 3 + 1

  const accountsYetToAirdrop = await program.account.escrow.all([
    {
      memcmp: {
        offset,
        bytes: bs58.encode([0]),
      },
    },
  ]);

  const accountsToAirdrop = accountsYetToAirdrop.filter((escrow) => {
    return escrow.account.completed;
  });

  return accountsToAirdrop;
}

async function airdrop() {
  const accountsToAirdrop = await getAccountsToAirdrop();
  console.log({ accountsToAirdrop });

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

async function main() {
  while (true) {
    try {
      await airdrop();
      await new Promise((resolve) => setTimeout(resolve, 1000));
    } catch (err) {
      // add error handling if needed
      throw err;
    }
  }
}

main();
