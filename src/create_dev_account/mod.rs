use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

use ed25519_dalek::Keypair;
use rand::{rngs::OsRng, Rng};

struct StringifyKeypair {
    public: String,
    secret: String,
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

pub async fn process() {
    let account_id = generate_accound_id();
    let keypair = generate_keypair();

    let helper_url = "https://helper.nearprotocol.com/account";

    let mut data = HashMap::new();
    data.insert("newAccountId", account_id.clone());
    data.insert("newAccountPublicKey", keypair.public.clone());

    let client = reqwest::Client::new();
    let res = client.post(helper_url).json(&data).send().await.unwrap();

    println!("{:?}", res);

    println!("accountId: {}", account_id);
    println!("public key: {}", keypair.public);
    println!("secret key: {}", keypair.secret);
}
