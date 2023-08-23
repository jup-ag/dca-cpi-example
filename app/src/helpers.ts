import {
  createAssociatedTokenAccountInstruction,
  getAssociatedTokenAddressSync,
} from '@solana/spl-token';
import { Connection, PublicKey, TransactionInstruction } from '@solana/web3.js';
import { BN } from '@coral-xyz/anchor';

const PDA_SEED = 'pda';

interface GetOrCreateATAResponse {
  ataPubKey: PublicKey;
  ix?: TransactionInstruction;
}

export async function getOrCreateATAInstruction(
  connection: Connection,
  tokenMint: PublicKey,
  owner: PublicKey,
  payer: PublicKey = owner,
  allowOwnerOffCurve = true,
): Promise<GetOrCreateATAResponse> {
  try {
    const toAccount = getAssociatedTokenAddressSync(
      tokenMint,
      owner,
      allowOwnerOffCurve,
    );

    const account = await connection.getAccountInfo(toAccount);

    if (account) return { ataPubKey: toAccount, ix: undefined };

    const ix = createAssociatedTokenAccountInstruction(
      payer,
      toAccount,
      owner,
      tokenMint,
    );

    return { ataPubKey: toAccount, ix };
  } catch (e) {
    /* handle error */
    console.error('Error::getOrCreateATAInstruction', e);
    throw e;
  }
}

export function derivePda(
  programId: PublicKey,
  user: PublicKey,
  inputMint: PublicKey,
  outputMint: PublicKey,
  uid: BN,
) {
  const uidBuffer = uid.toArrayLike(Buffer, 'le', 8);

  const [dcaPubKey] = PublicKey.findProgramAddressSync(
    [
      Buffer.from(PDA_SEED),
      user.toBuffer(),
      inputMint.toBuffer(),
      outputMint.toBuffer(),
      uidBuffer,
    ],
    programId,
  );

  return dcaPubKey;
}
