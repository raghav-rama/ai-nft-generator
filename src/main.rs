use std::env;
use std::error::Error;

use async_openai::{
    types::{
        ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, CreateImageRequestArgs,
        ImageSize, ResponseFormat, Role,
    },
    Client,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env::set_var(
        "OPENAI_API_KEY",
        "YOUR API KEY HERE",
    );
    let client = Client::new();
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model("gpt-3.5-turbo")
        .messages([
            ChatCompletionRequestMessageArgs::default()
                .role(Role::System)
                .content("You are a helpful assintant.")
                .build()?,
            ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content("Who won the world cup series in 2020?")
                .build()?,
            ChatCompletionRequestMessageArgs::default()
                .role(Role::Assistant)
                .content("The Los Angeles Dodgers won the Worldl Series in 2020")
                .build()?,
            ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content("Where was it played?")
                .build()?,
        ])
        .build()?;
    let response = client.chat().create(request).await?;
    println!("\nResponse: \n");
    for choice in response.choices {
        println!(
            "{}: Role {} Content: {:?}",
            choice.index, choice.message.role, choice.message.content
        )
    }
    let image_request = CreateImageRequestArgs::default()
        .prompt("Cats on sofa and carpet in living room")
        .n(2)
        .response_format(ResponseFormat::Url)
        .size(ImageSize::S256x256)
        .user("async-openai")
        .build()?;
    let image_response = client.images().create(image_request).await?;
    let paths = image_response.save("./data").await?;
    paths
        .iter()
        .for_each(|path| println!("Image file path: {}", path.display()));
    Ok(())
}
