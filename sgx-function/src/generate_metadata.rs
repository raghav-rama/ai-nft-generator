use crate::*;
use std::env;
use std::error::Error;

use futures::TryFutureExt;
use keys::get_openai_api_key;
pub use switchboard_utils::reqwest::{
    get,
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Client,
};

use ai_nft_generator::model::NftMetadataBorsh;
use serde::Deserialize;
use serde_json::json;

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
                    // "content": "This is a Metaplex NFT standard. It contains the following attributes:\n\nName: Metaplex NFT Standard\nSymbol: MPT\nDescription: This is a standard for NFTs on the Metaplex platform.\nImage: https://www.metaplex.com/images/logo.png\nAnimationUrl: https://www.metaplex.com/images/logo.png\nExternalUrl: https://www.metaplex.com\nAttributes: [\"trait_type\": \"category\", \"value\": \"image\"], [\"trait_type\": \"files\", \"value\": \"https://www.metaplex.com/images/logo.png\"], [\"trait_type\": \"files\", \"value\": \"https://www.metaplex.com/images/logo.png\"]\nProperties: [\"files\": [\"uri\": \"https://www.metaplex.com/images/logo.png\", \"type\": \"image/png\", \"cdn\": \"https://www.metaplex.com/images/logo.png\"], \"category\": \"image\"]\n\nGenerate some random NFT Data"
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
}
