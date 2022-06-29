use ed25519_dalek::Keypair;
use rand::{rngs::OsRng, Rng};
use std::{
    collections::HashMap,
    fmt,
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

use near_crypto::{PublicKey, SecretKey};
use near_primitives::types::AccountId;

#[derive(Debug, Clone)]
pub struct Account {
    pub account_id: AccountId,
    pub public_key: PublicKey,
    pub secret_key: SecretKey,
}

struct StringifyKeypair {
    public: String,
    secret: String,
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AccountID: {}\nPublic Key: {}\nPrivate Key: {}",
            self.account_id, self.public_key, self.secret_key
        )
    }
}

fn generate_keypair() -> StringifyKeypair {
    let mut csprng = OsRng {};
    let keypair: Keypair = Keypair::generate(&mut csprng);

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

pub async fn process() -> Account {
    let account_id = generate_accound_id();
    let keypair = generate_keypair();

    let helper_url = "https://helper.nearprotocol.com/account";

    let mut data = HashMap::new();
    data.insert("newAccountId", account_id.clone());
    data.insert("newAccountPublicKey", keypair.public.clone());

    let client = reqwest::Client::new();
    match client.post(helper_url).json(&data).send().await {
        Ok(result) => result,
        Err(err) => panic!("{}", err),
    };

    let parsed_account_id: AccountId = match account_id.parse() {
        Ok(result) => result,
        Err(err) => panic!("{}", err),
    };

    let parsed_public_key = match PublicKey::from_str(&keypair.public) {
        Ok(result) => result,
        Err(err) => panic!("{}", err),
    };

    let parsed_secret_key = match SecretKey::from_str(&keypair.secret) {
        Ok(result) => result,
        Err(err) => panic!("{}", err),
    };

    Account {
        account_id: parsed_account_id,
        public_key: parsed_public_key,
        secret_key: parsed_secret_key,
    }
}
