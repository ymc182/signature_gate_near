use super::*;
use ed25519_dalek::PublicKey;
use near_sdk::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PermissionMessage {
    pub expiration: u64,
    pub reward: u128,
    pub pub_key: String,
}

pub fn create_pub_key_from_bs58(bs58: String) -> PublicKey {
    let bytes = bs58::decode(bs58).into_vec().unwrap();
    PublicKey::from_bytes(&bytes).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::Keypair;
    use ed25519_dalek::Signer;
    use near_sdk::serde_json::json;
    use rand::rngs::OsRng;
    #[test]
    fn test_create_pub_key_object() {
        let mut csprng = OsRng {};
        let key_pair: Keypair = Keypair::generate(&mut csprng);

        let public_bs58 = bs58::encode(key_pair.public.to_bytes()).into_string();

        let pub_key_object_created_from_bs58 = create_pub_key_from_bs58(public_bs58.clone());

        assert_eq!(
            pub_key_object_created_from_bs58.to_bytes(),
            key_pair.public.to_bytes()
        );
    }

    #[test]
    fn test_signed_message() {
        let mut csprng = OsRng {};
        let admin_kp = Keypair::generate(&mut csprng);
        let user_kp = Keypair::generate(&mut csprng);

        let allowed_mint_message = PermissionMessage {
            expiration: 0,
            reward: 100,
            pub_key: bs58::encode(user_kp.public.to_bytes()).into_string(),
        };

        let message_string = json!(allowed_mint_message).to_string();

        let message_hash = env::sha256(message_string.as_bytes());

        let signature = admin_kp.sign(&message_hash);

        println!("signature: {:?}", signature.to_string());

        //retrieve message from signature
    }
}
