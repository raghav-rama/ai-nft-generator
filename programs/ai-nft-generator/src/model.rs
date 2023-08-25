use crate::*;

#[account(zero_copy(unsafe))]
pub struct MyProgramState {
    pub bump: u8,
    pub authority: Pubkey,
    pub function: Pubkey,
}

#[derive(Clone, Copy, AnchorSerialize, AnchorDeserialize)]
pub struct NftMetadataBorsh {
    pub oracle_timestamp: i64,
    pub key: Pubkey,
    pub update_authority: Pubkey,
    pub mint: Pubkey,
    pub name: [u8; 32],
    pub symbol: [u8; 10],
    pub uri: [u8; 200],
    pub seller_fee_basis_points: u16,
    pub primary_sale_happened: bool,
    pub is_mutable: bool,
}

#[repr(packed)]
#[zero_copy(unsafe)]
pub struct NftMetadata {
    pub oracle_timestamp: i64,
    pub key: Pubkey,
    pub update_authority: Pubkey,
    pub mint: Pubkey,
    pub name: [u8; 32],
    pub symbol: [u8; 10],
    pub uri: [u8; 200],
    pub seller_fee_basis_points: u16,
    pub primary_sale_happened: bool,
    pub is_mutable: bool,
}

impl From<NftMetadataBorsh> for NftMetadata {
    fn from(value: NftMetadataBorsh) -> Self {
        Self {
            oracle_timestamp: value.oracle_timestamp,
            key: value.key,
            update_authority: value.update_authority,
            mint: value.mint,
            name: value.name,
            symbol: value.symbol,
            uri: value.uri,
            seller_fee_basis_points: value.seller_fee_basis_points,
            primary_sale_happened: value.primary_sale_happened,
            is_mutable: value.is_mutable,
        }
    }
}

#[repr(packed)]
#[account(zero_copy(unsafe))]
pub struct MyOracleState {
    pub bump: u8,
    pub nft: NftMetadata,
}

impl MyProgramState {
    let LEN = 8;
}