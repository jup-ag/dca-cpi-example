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
