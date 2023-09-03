use crate::*;

#[account(zero_copy(unsafe))]
pub struct MyProgramState {
    pub bump: u8,
    pub authority: Pubkey,
    pub function: Pubkey,
}

#[allow(non_snake_case)]
#[derive(Clone, Copy, AnchorSerialize, AnchorDeserialize)]
pub struct NftMetadataBorsh {
    pub name: [u8; 100],
    pub symbol: [u8; 100],
    pub description: [u8; 100],
    pub image: [u8; 100],
    pub animationUrl: Option<[u8; 100]>,
    pub externalUrl: Option<[u8; 100]>,
}

#[repr(packed)]
#[zero_copy(unsafe)]
#[allow(non_snake_case)]
pub struct NftMetadata {
    pub name: [u8; 100],
    pub symbol: [u8; 100],
    pub description: [u8; 100],
    pub image: [u8; 100],
    pub animationUrl: Option<[u8; 100]>,
    pub externalUrl: Option<[u8; 100]>,
}

impl From<NftMetadataBorsh> for NftMetadata {
    fn from(value: NftMetadataBorsh) -> Self {
        Self {
            name: value.name.clone(),
            symbol: value.symbol.clone(),
            description: value.description.clone(),
            image: value.image.clone(),
            animationUrl: value.animationUrl.clone(),
            externalUrl: value.externalUrl.clone(),
        }
    }
}

#[repr(packed)]
#[account(zero_copy(unsafe))]
pub struct MyOracleState {
    pub bump: u8,
    pub nft: NftMetadataBorsh,
}

impl MyOracleState {
    pub fn update_nft_metadata(&mut self, nft: NftMetadataBorsh) -> Result<()> {
        self.nft = nft;
        Ok(())
    }
}
