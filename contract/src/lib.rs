use ed25519::PermissionMessage;
use ed25519_dalek::Verifier;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde_json::{self};
use near_sdk::{bs58, env, near_bindgen};

mod ed25519;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub verifier_pub_key: Option<String>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            verifier_pub_key: None,
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    pub fn get_verifier_pub_key(&self) -> String {
        self.verifier_pub_key
            .as_ref()
            .expect("Verifier public key is not set")
            .to_string()
    }

    pub fn set_verifier_pub_key(&mut self, verifier_pub_key: String) {
        self.verifier_pub_key = Some(verifier_pub_key);
    }

    pub fn verify_signature(&mut self, signature_hex: String, declared_message: String) -> bool {
        let verifier_pk_bs58 = self
            .verifier_pub_key
            .as_ref()
            .expect("Verifier public key is not set");
        let verifier_pk = ed25519::create_pub_key_from_bs58(verifier_pk_bs58.to_string());
        let msg = declared_message;

        let signature_bytes =
            hex::decode(signature_hex).expect("Signature is not valid hex string");
        let signature = ed25519_dalek::Signature::from_bytes(&signature_bytes);

        if signature.is_err() {
            env::panic_str("Signature is not valid");
        }

        let signature = signature.unwrap();

        let verified = verifier_pk.verify(msg.as_bytes(), &signature);

        if verified.is_err() {
            return false;
        }
        let _msg_permission: PermissionMessage = serde_json::from_str(&msg).unwrap();
        // use msg_permission to mint token
        true
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_create_new_contract() {}
}
