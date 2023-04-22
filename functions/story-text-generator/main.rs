use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {}

#[derive(Serialize)]
struct Response {
    status_code: i32,
    body: String,
}

#[derive(Serialize, Deserialize)]
struct JokeResponse {
    joke: String,
}

async fn function_handler(_event: LambdaEvent<Request>) -> Result<JokeResponse, Error> {
    let joke_response = get_joke().await?;
    Ok(joke_response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}

async fn get_joke() -> Result<JokeResponse, Error> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://icanhazdadjoke.com/")
        .header("Accept", "application/json")
        .send()
        .await?
        .json()
        .await?;

    return Ok(response);
}
