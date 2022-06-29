mod create_dev_account;
mod deploy;

#[tokio::main]
async fn main() {
    create_dev_account::process().await;
    deploy::process().await;
}
