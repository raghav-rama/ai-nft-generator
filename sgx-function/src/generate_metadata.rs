use crate::*;
use std::env;
use std::error::Error;

use keys::get_openai_api_key;
pub use switchboard_utils::reqwest;

use ai_nft_generator::model::NftMetadataBorsh;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Default, Debug, Deserialize, Clone)]
pub struct MetaplexNftStandard {
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub image: String,
    pub animationUrl: String,
    pub externalUrl: String,
    pub attributes: Vec<Attribute>,
    pub properties: Properties,
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
    pub type_field: String,
    pub cdn: Option<String>,
}

impl MetaplexNftStandard {
    pub async fn get_data() -> std::result::Result<MetaplexNftStandard, SwitchboardClientError> {
        let api_key = get_openai_api_key();
        let auth_header = format!("Bearer {}", api_key);
        let url = "https://api.openai.com/v1/engines/gpt-3.5-turbo/completions";
        let headers = vec![
            ("Authorization", auth_header.as_str()),
            ("Content-Type", "application/json"),
        ];
        let body = r#"{
            "prompt": "This is a Metaplex NFT standard. It contains the following attributes:\n\nName: Metaplex NFT Standard\nSymbol: MPT\nDescription: This is a standard for NFTs on the Metaplex platform.\nImage: https://www.metaplex.com/images/logo.png\nAnimationUrl: https://www.metaplex.com/images/logo.png\nExternalUrl: https://www.metaplex.com\nAttributes: [\"trait_type\": \"category\", \"value\": \"image\"], [\"trait_type\": \"files\", \"value\": \"https://www.metaplex.com/images/logo.png\"], [\"trait_type\": \"files\", \"value\": \"https://www.metaplex.com/images/logo.png\"]\nProperties: [\"files\": [\"uri\": \"https://www.metaplex.com/images/logo.png\", \"type\": \"image/png\", \"cdn\": \"https://www.metaplex.com/images/logo.png\"], \"category\": \"image\"]\n\nGenerate some random NFT Data",
            "max_tokens": 512,
            "temperature": 0.0,
            "top_p": 1.0,
            "n": 1,
            "stream": false,
            "logprobs": null,
            "stop": "\n"
        }"#;
        let response = reqwest::get(url).await.unwrap().json::<MetaplexNftStandard>().await.unwrap();
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
}
