use crate::*;
use keys::get_openai_api_key;
use std::error::Error;
pub use switchboard_utils::reqwest::{
    get,
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Client,
};

use async_openai::{
    types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
    Client as OpenAiClient,
};

use ai_nft_generator::{model::NftMetadataBorsh, GenerateNftParams, NftMetadata};
use serde::Deserialize;
use serde_json::json;

pub fn ix_discriminator(name: &str) -> [u8; 8] {
    let preimage = format!("global:{}", name);
    let mut sighash = [0u8; 8];
    sighash.copy_from_slice(
        &anchor_lang::solana_program::hash::hash(preimage.as_bytes()).to_bytes()[..8],
    );
    sighash
}

#[allow(non_snake_case)]
#[derive(Default, Debug, Deserialize, Clone)]
pub struct MetaplexNftStandard {
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub image: String,
    pub animationUrl: Option<String>,
    pub externalUrl: Option<String>,
    pub attributes: Option<Vec<Attribute>>,
    pub properties: Option<Properties>,
}

#[derive(Default, Debug, Deserialize, Clone)]
pub struct Attribute {
    pub trait_type: String,
    pub value: String,
}

#[derive(Default, Debug, Deserialize, Clone)]
pub struct Properties {
    pub files: Vec<File>,
    pub category: String,
}

#[derive(Default, Debug, Deserialize, Clone)]
pub struct File {
    pub uri: String,
    #[serde(rename = "type")]
    pub file_type: String,
    pub cdn: Option<String>,
}

impl Into<NftMetadataBorsh> for MetaplexNftStandard {
    fn into(self) -> NftMetadataBorsh {
        NftMetadataBorsh {
            name: self.name.as_bytes().try_into().unwrap(),
            symbol: self.symbol.as_bytes().try_into().unwrap(),
            description: self.description.as_bytes().try_into().unwrap(),
            image: self.image.as_bytes().try_into().unwrap(),
            animationUrl: match self.animationUrl {
                Some(url) => Some(url.as_bytes().try_into().unwrap()),
                None => None,
            },
            externalUrl: match self.externalUrl {
                Some(url) => Some(url.as_bytes().try_into().unwrap()),
                None => None,
            },
        }
    }
}

impl Into<NftMetadata> for MetaplexNftStandard {
    fn into(self) -> NftMetadata {
        NftMetadata {
            name: self.name.as_bytes().try_into().unwrap(),
            symbol: self.symbol.as_bytes().try_into().unwrap(),
            description: self.description.as_bytes().try_into().unwrap(),
            image: self.image.as_bytes().try_into().unwrap(),
            animationUrl: match self.animationUrl {
                Some(url) => Some(url.as_bytes().try_into().unwrap()),
                None => None,
            },
            externalUrl: match self.externalUrl {
                Some(url) => Some(url.as_bytes().try_into().unwrap()),
                None => None,
            },
        }
    }
}
// "content": "This is a Metaplex NFT standard. It contains the following attributes:\n\nName: Metaplex NFT Standard\nSymbol: MPT\nDescription: This is a standard for NFTs on the Metaplex platform.\nImage: https://www.metaplex.com/images/logo.png\nAnimationUrl: https://www.metaplex.com/images/logo.png\nExternalUrl: https://www.metaplex.com\nAttributes: [\"trait_type\": \"category\", \"value\": \"image\"], [\"trait_type\": \"files\", \"value\": \"https://www.metaplex.com/images/logo.png\"], [\"trait_type\": \"files\", \"value\": \"https://www.metaplex.com/images/logo.png\"]\nProperties: [\"files\": [\"uri\": \"https://www.metaplex.com/images/logo.png\", \"type\": \"image/png\", \"cdn\": \"https://www.metaplex.com/images/logo.png\"], \"category\": \"image\"]\n\nGenerate some random NFT Data"
impl MetaplexNftStandard {
    pub async fn get_data() -> std::result::Result<MetaplexNftStandard, SwitchboardClientError> {
        let api_key = get_openai_api_key();
        let auth_header = format!("Bearer {}", api_key);
        let url = "https://api.openai.com/v1/chat/completions";
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&auth_header).unwrap());
        let data = json!({
            "model": "gpt-3.5-turbo",
            "messages": [
                {
                    "role": "system",
                    "content": "You are an NFT nerd"
                },
                {
                    "role": "user",
                    "content": "Name some cool NFT Projects"
                },
            ]
        });
        let client = Client::new();
        let res = client
            .post(url)
            .headers(headers)
            .json(&data)
            .send()
            .await
            .unwrap();
        // .json::<MetaplexNftStandard>()
        // .await
        // .unwrap();
        println!("Response: {:?}", res);

        let response = get(url)
            .await
            .unwrap()
            .json::<MetaplexNftStandard>()
            .await
            .unwrap();
        Ok(MetaplexNftStandard {
            name: response.name,
            symbol: response.symbol,
            description: response.description,
            image: response.image,
            animationUrl: response.animationUrl,
            externalUrl: response.externalUrl,
            attributes: response.attributes,
            properties: response.properties,
        })
    }
    pub async fn chat_completions(
        prompt: String,
        nft_url: String,
    ) -> std::result::Result<Self, Box<dyn Error>> {
        let client = OpenAiClient::new();
        let request = CreateChatCompletionRequestArgs::default()
            .max_tokens(512u16)
            .model("gpt-3.5-turbo")
            .messages([
                ChatCompletionRequestMessageArgs::default()
                    .role(Role::System)
                    .content(format!("This is the Metaplex NFT Standard. Generate some random NFT Data, in this format as a json only(key, value pair)\n\n
                    name: nft name\n
                    symbol: initials\n
                    description: any random description\n
                    image: {}\n
                    animationUrl: https://www.metaplex.com/images/logo.png\n
                    externalUrl: https://www.metaplex.com\n
                    attributes: [\"trait_type\": \"category\", \"value\": \"image\"], [\"trait_type\": \"files\", \"value\": \"https://www.metaplex.com/images/logo.png\"], [\"trait_type\": \"files\", \"value\": \"https://www.metaplex.com/images/logo.png\"]\n
                    properties: \"files\": [\"uri\": \"https://www.metaplex.com/images/logo.png\", \"type_field\": \"image/png\", \"cdn\": \"https://www.metaplex.com/images/logo.png\"], \"category\": \"image\"\n", nft_url))
                    .build()
                    .unwrap(),
                ChatCompletionRequestMessageArgs::default()
                    .role(Role::User)
                    .content(prompt)
                    .build()
                    .unwrap(),
            ])
            .build()?;
        let mut response = client.chat().create(request).await?;
        let metadata = response.choices[0]
            .message
            .content
            .as_mut()
            .unwrap()
            .replace("\n", "");
        Ok(
            match serde_json::from_str::<MetaplexNftStandard>(metadata.as_str()) {
                Ok(result) => result,
                Err(e) => {
                    println!("metadata: {:?}", metadata);
                    panic!("Error: {:?}", e)
                }
            },
        )
    }
    pub fn generate_ixns(self, runner: &FunctionRunner) -> Vec<Instruction> {
        let generate_nft_params = GenerateNftParams { nft: self.into() };
        let (_, _) =
            Pubkey::find_program_address(&[b"ai-nft-generator-oracle"], &ai_nft_generator::ID);
        let (oracle_pubkey, _) =
            Pubkey::find_program_address(&[b"ai-nft-generator-oracle"], &ai_nft_generator::ID);
        let ixn = Instruction {
            program_id: ai_nft_generator::ID,
            accounts: vec![
                AccountMeta {
                    pubkey: oracle_pubkey,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: runner.function,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: runner.signer,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: runner.function_request_key.unwrap(),
                    is_signer: false,
                    is_writable: false,
                }
            ],
            data: [
                ix_discriminator("mint_ai_nft").to_vec(),
                generate_nft_params.try_to_vec().unwrap(),
            ]
            .concat(),
        };
        vec![ixn]
    }
}
