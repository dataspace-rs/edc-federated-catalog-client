#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
  use clap::Parser;

  #[derive(clap::Parser)]
  struct Args {
    #[clap(long, short, env)]
    endpoint: String,
    #[clap(long, short='t', env)]
    bearer_token: Option<String>,
  }

  let args = Args::parse();

  let client = edc_federated_catalog_client::FederatedCatalogClient::new(
    reqwest::Client::new(),
    args.endpoint,
    args.bearer_token,
    edc_federated_catalog_client::FederatedCatalogClientVersion::V4,
  );

  let offers = client.list_offers().await.unwrap();
  println!("{:?}", offers);
}

#[cfg(target_arch = "wasm32")]
fn main() {}
