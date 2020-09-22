use twitch_api2::{helix::webhooks, HelixClient};
use twitch_oauth2::{AccessToken, UserToken};

fn main() {
    use std::error::Error;
    if let Err(err) = run() {
        println!("Error: {}", err);
        let mut e: &'_ dyn Error = err.as_ref();
        while let Some(cause) = e.source() {
            println!("Caused by: {:?}", cause);
            e = cause;
        }
    }
}

#[tokio::main]
async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    dotenv::dotenv().unwrap();
    let mut args = std::env::args().skip(1);
    let token = UserToken::from_existing(
        twitch_oauth2::client::reqwest_http_client,
        std::env::var("TWITCH_TOKEN")
            .ok()
            .or_else(|| args.next())
            .map(AccessToken::new)
            .expect("Please set env: TWITCH_TOKEN or pass token as first argument"),
        None,
    )
    .await
    .unwrap();

    let client: HelixClient<'static, reqwest::Client> = HelixClient::new();

    let req = webhooks::hub::Hub::new(webhooks::StreamChanged::builder().user_id("1234").build());
    let body = webhooks::hub::HubBody::builder().callback("https://example.com?lol=hahah".to_string()).lease_seconds(10000).build();
    let response = client.req_post(req, body, &token).await.unwrap();

    let req = webhooks::hub::Hub::new(webhooks::StreamChanged::builder().user_id("1234").build());
    let body = webhooks::hub::HubBody::builder().callback("https://example.com?lol=hahah".to_string()).lease_seconds(10000).build();
    let response = client.req_post(req, body, &token).await.unwrap();

    Ok(())
}
