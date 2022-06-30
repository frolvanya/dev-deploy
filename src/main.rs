use clap::Parser;

mod create_dev_account;
mod deploy;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Smart contract path
    #[clap(short, long, value_parser)]
    file: String,
}

#[tokio::main]
async fn main() {
    let contract_path = Args::parse().file;

    let account_info = match create_dev_account::process().await {
        Ok(info) => info,
        Err(err) => panic!("{}", err),
    };
    match deploy::process(account_info, contract_path).await {
        Err(err) => panic!("{}", err),
        Ok(_) => {}
    };
}
