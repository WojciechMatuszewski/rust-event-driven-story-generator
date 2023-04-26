use std::time::Duration;

use aws_sdk_s3::presigning;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Debug)]
enum TaskStatus {
    COMPLETED,
    FAILED,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Message {
    task_id: String,
    task_status: TaskStatus,
    output_uri: String,
}

#[derive(Serialize, Deserialize)]
struct Output {
    message: String,
}

async fn function_handler(
    event: LambdaEvent<aws_lambda_events::sns::SnsEvent>,
) -> Result<Output, Error> {
    let record = event.payload.records.get(0).unwrap();
    let message: Message = serde_json::from_str(&record.sns.message)?;

    println!("Serialized the message and got the record");

    let bucket_name = std::env::var("STORY_AUDIO_BUCKET_NAME")?;
    let task_token = get_task_token(&message, &bucket_name);
    let key = get_audio_key(&message, &bucket_name);

    println!("Got the task token and the bucket name");

    let config = aws_config::load_from_env().await;
    let sfn_client = aws_sdk_sfn::client::Client::new(&config);

    println!("Getting the presigned URL");

    let s3_client = aws_sdk_s3::client::Client::new(&config);

    let expires_in = Duration::from_secs(60 * 60);
    let presigned_url = s3_client
        .get_object()
        .bucket(bucket_name)
        .key(key)
        .presigned(presigning::PresigningConfig::expires_in(expires_in)?)
        .await?
        .to_http_request::<Option<String>>(None)?;

    println!("Got the presigned URL foo bar");

    match message.task_status {
        TaskStatus::COMPLETED => {
            sfn_client
                .send_task_success()
                .set_task_token(Some(task_token))
                .set_output(Some(
                    json!({"location": presigned_url.uri().to_string()}).to_string(),
                ))
                .send()
                .await?;
        }
        _ => {
            sfn_client
                .send_task_failure()
                .set_task_token(Some(task_token))
                .send()
                .await?;
        }
    }

    println!("Sending the response back");

    return Ok(Output {
        message: "Done".to_string(),
    });
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

fn get_audio_key(message: &Message, bucket_name: &str) -> String {
    let bucket_uri = format!("s3://{}/", bucket_name);
    let key = message.output_uri.trim_start_matches(bucket_uri.as_str());
    return key.to_string();
}

fn get_task_token(message: &Message, bucket_name: &str) -> String {
    let bucket_uri = format!("s3://{}/", bucket_name);

    let task_token = message
        .output_uri
        .trim_start_matches(bucket_uri.as_str())
        .trim_end_matches(format!("/output.{}.mp3", message.task_id).as_str());

    return task_token.into();
}
