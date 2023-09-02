pub use switchboard_solana::prelude::*;

pub mod generate_metadata;
pub use generate_metadata::*;

pub mod keys;
pub use keys::*;
use switchboard_solana::solana_sdk::commitment_config::CommitmentConfig;

use std::env;
use std::error::Error;

// use async_openai::{
//     types::{
//         ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, CreateImageRequestArgs,
//         ImageSize, ResponseFormat, Role,
//     },
//     Client,
// };
// use crate::sgx_function::model::NftMetadataBorsh;

#[tokio::main(worker_threads = 12)]
async fn main() {
    set_openai_api_key().unwrap();
    env::set_var(
        "FUNCTION_KEY",
        "BvvrSwT1KFwXf8v4E7ZA1L7jdFTYzt1WcqBV8FfDtTJx",
    );
    env::set_var("PAYER", "5krot8UnMEqoDkAc72e7pqaEaF5hxGmbDNowMmPiCDmb");
    env::set_var("FUNCTION_DATA", "x");
    env::set_var("FUNCTION_REQUEST_KEY", "x");
    env::set_var("FUNCTION_REQUEST_DATA", "x");
    env::set_var("VERIFIER", "5krot8UnMEqoDkAc72e7pqaEaF5hxGmbDNowMmPiCDmb");
    env::set_var(
        "REWARD_RECEIVER",
        "5krot8UnMEqoDkAc72e7pqaEaF5hxGmbDNowMmPiCDmb",
    );
    env::set_var(
        "VERIFIER_ENCLAVE_SIGNER",
        "5krot8UnMEqoDkAc72e7pqaEaF5hxGmbDNowMmPiCDmb",
    );
    env::set_var(
        "QUEUE_AUTHORITY",
        "5krot8UnMEqoDkAc72e7pqaEaF5hxGmbDNowMmPiCDmb",
    );
    let runner = FunctionRunner::new_from_cluster(
        Cluster::Devnet,
        Some(CommitmentConfig {
            commitment: solana_sdk::commitment_config::CommitmentLevel::Confirmed,
        }),
    )
    .unwrap();
    match MetaplexNftStandard::chat_completions().await {
        Ok(_) => {
            println!("Deserization Succeeded!");
        },
        Err(e) => {
            println!("error: \n{:?}", e);
        }
    }
    let ixns = vec![];
    runner.emit(ixns).await.unwrap();
    // let nft = MetaplexNftStandard::get_data().await.unwrap();

    // let image_request = CreateImageRequestArgs::default()
    //     .prompt("Cats on sofa and carpet in living room")
    //     .n(2)
    //     .response_format(ResponseFormat::Url)
    //     .size(ImageSize::S256x256)
    //     .user("async-openai")
    //     .build()?;
    // let image_response = client.images().create(image_request).await?;
    // let paths = image_response.save("./data").await?;
    // paths
    //     .iter()
    //     .for_each(|path| println!("Image file path: {}", path.display()));
}
