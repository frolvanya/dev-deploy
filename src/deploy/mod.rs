use near_jsonrpc_client::methods::tx::TransactionInfo;
use near_jsonrpc_client::{methods, JsonRpcClient};
use near_primitives::transaction::{Action, DeployContractAction};

use tokio::time;

use crate::create_dev_account;

mod utils;

pub(crate) async fn process(
    account_info: create_dev_account::Account,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = JsonRpcClient::connect("https://rpc.testnet.near.org");

    // let signer_account_id = utils::input("Enter the signer Account ID: ")?.parse()?;
    // let signer_public_key = utils::input("Enter the signer's public key: ")?;
    // let signer_secret_key = utils::input("Enter the signer's private key: ")?.parse()?;

    let signer = near_crypto::InMemorySigner::from_secret_key(
        account_info.account_id.clone(),
        account_info.secret_key.clone(),
    );

    let code = std::fs::read(utils::input("Enter the file location of the contract: ")?)?;

    let transaction = near_primitives::transaction::Transaction {
        signer_id: account_info.account_id.clone(),
        public_key: account_info.public_key.clone(),
        nonce: 0,
        receiver_id: account_info.account_id.clone(),
        block_hash: Default::default(),
        actions: vec![Action::DeployContract(DeployContractAction { code })],
    };

    let request = methods::broadcast_tx_async::RpcBroadcastTxAsyncRequest {
        signed_transaction: transaction.sign(&signer),
    };

    let sent_at = time::Instant::now();
    let tx_hash = client.call(request).await?;

    loop {
        let response = client
            .call(methods::tx::RpcTransactionStatusRequest {
                transaction_info: TransactionInfo::TransactionId {
                    hash: tx_hash,
                    account_id: signer.account_id.clone(),
                },
            })
            .await;
        let received_at = time::Instant::now();
        let delta = (received_at - sent_at).as_secs();

        if delta > 60 {
            Err("time limit exceeded for the transaction to be recognized")?;
        }

        match response {
            Err(err) => panic!("{}", err),
            Ok(response) => {
                println!("response gotten after: {}s", delta);
                println!("response: {:#?}", response);
                break;
            }
        }
    }

    Ok(())
}
