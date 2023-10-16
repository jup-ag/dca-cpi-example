pub const ESCROW_SEED: &[u8] = b"escrow";

// DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263
pub const BONK_MINT: [u8; 32] = [
    188, 7, 197, 110, 96, 173, 61, 63, 23, 115, 130, 234, 198, 84, 143, 186, 31, 211, 44, 253, 144,
    202, 2, 179, 231, 207, 161, 133, 253, 206, 115, 152,
];

// hardcoded upgrade authority
// also used to create the vault token account where reward bonuses will be funded from
// auuSyC1KwkNwZv7BKxXXrWTjym3jW6YpFqeovpZ7i5h
pub const AUTHORITY_PK: [u8; 32] = [
    8, 175, 252, 90, 25, 25, 166, 33, 33, 165, 63, 97, 221, 7, 221, 150, 70, 204, 4, 222, 83, 55,
    230, 233, 3, 53, 73, 89, 76, 51, 198, 100,
];

// authority signer for the rewards vault
// 6QjkBcZmEyFZrS5egbfisKTWmo3FFXupjBW1qvorrT6v
// PublicKey.findProgramAddressSync([Buffer.from('vault')], escrowProgramId)
pub const VAULT_SIGNER_PDA: [u8; 32] = [
    80, 95, 31, 232, 94, 195, 114, 133, 115, 74, 63, 38, 209, 230, 75, 63, 232, 28, 247, 129, 242,
    197, 181, 78, 233, 155, 239, 247, 235, 205, 152, 183,
];

pub const ACCEPTED_DURATION_SECONDS: [u32; 4] = [
    60 * 5,  // 5 minutes for testing
    1209600, // 14 days
    2592000, // 30 days
    7776000, // 90 days
];

pub const AIRDROP_BPS: [u64; 4] = [
    200,  // 2% @ 5 minutes for testing
    200,  // 2% @ 14 days
    500,  // 5% @ 30 days
    1000, // 10% @ 90 days
];
