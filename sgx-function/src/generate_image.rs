use async_openai::{types::{CreateImageRequestArgs, ImageSize, ResponseFormat, ImageData},Client};
use std::error::Error;
use crate::set_openai_api_key;

pub async fn generate_image(prompt: String) -> Result<String, Box<dyn Error>> {
    set_openai_api_key()?;
    let client = Client::new();
    let request = CreateImageRequestArgs::default()
        .prompt(prompt)
        .n(1)
        .response_format(ResponseFormat::Url)
        .size(ImageSize::S512x512)
        .user("0x69x0")
        .build()?;
    let response = client.images().create(request).await?;
    println!("{:#?}", response.data);
    match &*response.data[0] {
        ImageData::Url(url) => Ok(url.to_string()),
        ImageData::B64Json(base64) => Ok(base64.to_string()),
    }
}
