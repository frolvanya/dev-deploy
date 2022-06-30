use crate::create_dev_account;

use anyhow::Result;

pub(crate) async fn process(
    account_info: create_dev_account::Account,
    contract_path: String,
) -> Result<()> {
    let client = near_jsonrpc_client::JsonRpcClient::connect("https://rpc.testnet.near.org");

    let signer = near_crypto::InMemorySigner::from_secret_key(
        account_info.account_id.clone(),
        account_info.secret_key.clone(),
    );

    let code = match std::fs::read(contract_path.clone()) {
        Ok(result) => result,
        Err(err) => return Err(anyhow::anyhow!(err)),
    };

    println!("Starting deployment. Account ID: {}, node: https://rpc.testnet.near.org, helper: https://helper.testnet.near.org, file: {}\n", account_info.account_id, contract_path);

    let access_key_query_response = match client
        .call(near_jsonrpc_client::methods::query::RpcQueryRequest {
            block_reference: near_primitives::types::BlockReference::latest(),
            request: near_primitives::views::QueryRequest::ViewAccessKey {
                account_id: signer.account_id.clone(),
                public_key: signer.public_key.clone(),
            },
        })
        .await
    {
        Ok(result) => result,
        Err(err) => return Err(anyhow::anyhow!(err)),
    };

    let current_nonce = match access_key_query_response.kind {
        near_jsonrpc_primitives::types::query::QueryResponseKind::AccessKey(access_key) => {
            access_key.nonce
        }
        _ => return Err(anyhow::anyhow!("failed to extract current nonce")),
    };

    let unsigned_transaction = near_primitives::transaction::Transaction {
        signer_id: account_info.account_id.clone(),
        public_key: account_info.public_key.clone(),
        nonce: current_nonce + 1,
        receiver_id: account_info.account_id.clone(),
        block_hash: access_key_query_response.block_hash,
        actions: vec![near_primitives::transaction::Action::DeployContract(
            near_primitives::transaction::DeployContractAction { code },
        )],
    };

    let request = near_jsonrpc_client::methods::broadcast_tx_commit::RpcBroadcastTxCommitRequest {
        signed_transaction: unsigned_transaction.sign(&signer),
    };

    let response = match client.call(request).await {
        Ok(result) => result,
        Err(_) => return Err(anyhow::anyhow!("failed to extract current nonce")),
    };

    match response.status {
        near_primitives::views::FinalExecutionStatus::NotStarted => {
            println!("Deployment doesn't started")
        }
        near_primitives::views::FinalExecutionStatus::Started => {
            println!("Starting deploying to {}", account_info.account_id)
        }
        near_primitives::views::FinalExecutionStatus::Failure(_) => {
            println!("Failed deploying to {}", account_info.account_id)
        }
        near_primitives::views::FinalExecutionStatus::SuccessValue(_) => println!(
            "Transaction ID: {0}
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/{0}

Done deploying to {1}",
            response.transaction.hash, account_info.account_id
        ),
    }

    Ok(())
}
