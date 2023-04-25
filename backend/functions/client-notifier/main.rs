use aws_sdk_iotdataplane::primitives::Blob;
use futures::future::join_all;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum Status {
    Starting,
    Failed,
    TextGenerated,
    Completed,
}

#[derive(Serialize, Deserialize, Debug)]
struct Item {
    pk: String,
    status: Status,
    text: Option<String>,
    audio: Option<String>,
}

async fn function_handler(
    event: LambdaEvent<aws_lambda_events::dynamodb::Event>,
) -> Result<(), Error> {
    let config = aws_config::load_from_env().await;

    let records = event.payload.records;
    let handles = records.iter().map(|record| {
        let client = aws_sdk_iotdataplane::client::Client::new(&config);
        let to_pass_record = record.clone();

        return tokio::spawn(async move { process_record(client, to_pass_record).await });
    });

    join_all(handles).await;

    return Ok(());
}

async fn process_record(
    client: aws_sdk_iotdataplane::client::Client,
    record: aws_lambda_events::dynamodb::EventRecord,
) {
    let item: Item = serde_dynamo::from_item(record.change.new_image).unwrap();
    let payload = serde_json::to_string(&item).unwrap();

    client
        .publish()
        .topic("story/1")
        .qos(1)
        .payload(Blob::new(payload.as_bytes()))
        .send()
        .await
        .unwrap();
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
