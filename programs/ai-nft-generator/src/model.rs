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
            key: value.key.clone(),
            update_authority: value.update_authority.clone(),
            mint: value.mint.clone(),
            name: value.name.clone(),
            symbol: value.symbol.clone(),
            uri: value.uri.clone(),
            seller_fee_basis_points: value.seller_fee_basis_points.clone(),
            primary_sale_happened: value.primary_sale_happened.clone(),
            is_mutable: value.is_mutable.clone(),
        }
    }
}

#[repr(packed)]
#[account(zero_copy(unsafe))]
pub struct MyOracleState {
    pub bump: u8,
    pub nft: NftMetadata,
}

impl MyOracleState {
    pub fn update_nft_metadata(&mut self, nft: NftMetadata) -> Result<()> {
        self.nft = nft;
        Ok(())
    }
}
