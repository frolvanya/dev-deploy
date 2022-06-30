use anyhow::Result;
use near_primitives::types::AccountId;
use rand::{rngs::OsRng, Rng};
use std::{
    collections::HashMap,
    fmt,
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Clone)]
pub(crate) struct Account {
    pub(crate) account_id: near_primitives::types::AccountId,
    pub(crate) public_key: near_crypto::PublicKey,
    pub(crate) secret_key: near_crypto::SecretKey,
}

struct StringifyKeypair {
    public: String,
    secret: String,
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AccountID: {}\nPublic Key: {}\nPrivate Key: {}\n",
            self.account_id, self.public_key, self.secret_key
        )
    }
}

fn generate_keypair() -> StringifyKeypair {
    let mut csprng = OsRng {};
    let keypair = ed25519_dalek::Keypair::generate(&mut csprng);

    let public_key_str = format!("ed25519:{}", bs58::encode(&keypair.public).into_string());
    let secret_keypair_str = format!(
        "ed25519:{}",
        bs58::encode(&keypair.to_bytes()).into_string()
    );

    StringifyKeypair {
        public: public_key_str,
        secret: secret_keypair_str,
    }
}

fn generate_accound_id() -> String {
    let epoch_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let random_number = rand::thread_rng().gen_range(10000000000000i64, 100000000000000i64);

    format!("dev-{}-{}", epoch_time, random_number)
}

pub(crate) async fn process() -> Result<Account> {
    let helper_url = "https://helper.nearprotocol.com/account";

    let account_id = generate_accound_id();
    let keypair = generate_keypair();

    let mut data = HashMap::new();
    data.insert("newAccountId", account_id.clone());
    data.insert("newAccountPublicKey", keypair.public.clone());

    let client = reqwest::Client::new();
    match client.post(helper_url).json(&data).send().await {
        Ok(result) => result,
        Err(err) => return Err(anyhow::anyhow!(err)),
    };

    let parsed_account_id = match account_id.parse::<AccountId>() {
        Ok(result) => result,
        Err(err) => return Err(anyhow::anyhow!(err)),
    };

    let parsed_public_key = match near_crypto::PublicKey::from_str(&keypair.public) {
        Ok(result) => result,
        Err(err) => return Err(anyhow::anyhow!(err)),
    };

    let parsed_secret_key = match near_crypto::SecretKey::from_str(&keypair.secret) {
        Ok(result) => result,
        Err(err) => return Err(anyhow::anyhow!(err)),
    };

    Ok(Account {
        account_id: parsed_account_id,
        public_key: parsed_public_key,
        secret_key: parsed_secret_key,
    })
}
