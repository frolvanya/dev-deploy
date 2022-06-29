mod create_dev_account;
mod deploy;

#[tokio::main]
async fn main() {
    let account_info = create_dev_account::process().await;
    println!("{}", account_info);

    deploy::process(account_info).await.unwrap();
}
