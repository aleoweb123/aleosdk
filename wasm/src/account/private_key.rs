// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the Aleo SDK library.

// The Aleo SDK library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Aleo SDK library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Aleo SDK library. If not, see <https://www.gnu.org/licenses/>.

use crate::{
    account::{Address, PrivateKeyCiphertext, Signature, ViewKey},
    record::RecordCiphertext,
    types::{CurrentNetwork, Encryptor, Environment, FromBytes, PrimeField, PrivateKeyNative, ToBytes},
};

use core::{convert::TryInto, fmt, ops::Deref, str::FromStr};
use rand::{rngs::StdRng, SeedableRng};
use wasm_bindgen::prelude::*;

use aleo_rust::{Network, Plaintext, Record};
use serde::{Deserialize, Serialize};

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrivateKey(PrivateKeyNative);

#[derive(Serialize)]
pub struct OldRecordData<N: Network> {
    record: Record<N, Plaintext<N>>,
    transactionid: N::TransitionID,
    serial_number: aleo_rust::Field<aleo_rust::Testnet3>,
}

#[derive(Serialize)]
pub struct RecordData<N: Network> {
    record: Record<N, Plaintext<N>>,
    identifier: String,
    serial_number: String,
    program_id: String,
    height: u32,
    timestamp: i64,
    block_hash: String,
    transaction_id: String,
    transition_id: String,
    function_name: String,
    output_index: u8,
    input: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct RecordOrgData {
    record_ciphertext: String,
    identifier: String,
    program_id: String,
    height: u32,
    timestamp: i64,
    block_hash: String,
    transaction_id: String,
    transition_id: String,
    function_name: String,
    output_index: u8,
    input: Option<Vec<String>>,
}

#[wasm_bindgen]
impl PrivateKey {
    /// Generate a new private key
    #[wasm_bindgen(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        Self(PrivateKeyNative::new(&mut StdRng::from_entropy()).unwrap())
    }

    /// Get a private key from a series of unchecked bytes
    pub fn from_seed_unchecked(seed: &[u8]) -> PrivateKey {
        console_error_panic_hook::set_once();
        // Cast into a fixed-size byte array. Note: This is a **hard** requirement for security.
        let seed: [u8; 32] = seed.try_into().unwrap();
        // Recover the field element deterministically.
        let field = <CurrentNetwork as Environment>::Field::from_bytes_le_mod_order(&seed);
        // Cast and recover the private key from the seed.
        Self(PrivateKeyNative::try_from(FromBytes::read_le(&*field.to_bytes_le().unwrap()).unwrap()).unwrap())
    }

    /// Create a private key from a string representation
    ///
    /// This function will fail if the text is not a valid private key
    pub fn from_string(private_key: &str) -> Result<PrivateKey, String> {
        Self::from_str(private_key).map_err(|_| "Invalid private key".to_string())
    }

    /// Get a string representation of the private key
    ///
    /// This function should be used very carefully as it exposes the private key plaintext
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    /// Get the view key corresponding to the private key
    pub fn to_view_key(&self) -> ViewKey {
        ViewKey::from_private_key(self)
    }

    /// Get the address corresponding to the private key
    pub fn to_address(&self) -> Address {
        Address::from_private_key(self)
    }

    /// Sign a message with the private key
    pub fn sign(&self, message: &[u8]) -> Signature {
        Signature::sign(self, message)
    }

    /// Get a private key ciphertext using a secret.
    ///
    /// The secret is sensitive and will be needed to decrypt the private key later, so it should be stored securely
    #[wasm_bindgen(js_name = newEncrypted)]
    pub fn new_encrypted(secret: &str) -> Result<PrivateKeyCiphertext, String> {
        let key = Self::new();
        let ciphertext =
            Encryptor::encrypt_private_key_with_secret(&key, secret).map_err(|_| "Encryption failed".to_string())?;
        Ok(PrivateKeyCiphertext::from(ciphertext))
    }

    /// Encrypt the private key with a secret.
    ///
    /// The secret is sensitive and will be needed to decrypt the private key later, so it should be stored securely
    #[wasm_bindgen(js_name = toCiphertext)]
    pub fn to_ciphertext(&self, secret: &str) -> Result<PrivateKeyCiphertext, String> {
        let ciphertext =
            Encryptor::encrypt_private_key_with_secret(self, secret).map_err(|_| "Encryption failed".to_string())?;
        Ok(PrivateKeyCiphertext::from(ciphertext))
    }

    /// Get private key from a private key ciphertext using a secret.
    #[wasm_bindgen(js_name = fromPrivateKeyCiphertext)]
    pub fn from_private_key_ciphertext(ciphertext: &PrivateKeyCiphertext, secret: &str) -> Result<PrivateKey, String> {
        let private_key = Encryptor::decrypt_private_key_with_secret(ciphertext, secret)
            .map_err(|_| "Decryption failed".to_string())?;
        Ok(Self::from(private_key))
    }

    #[wasm_bindgen(js_name = "decryptrecords")]
    pub fn decrypt_records(&self, recordstext: &str) -> Result<String, String> {
        let record_org_datas: Vec<RecordOrgData> = serde_json::from_str(recordstext).unwrap_or_default();
        let mut records = Vec::new();
        for record_org in record_org_datas {
            if let Ok(record) = RecordCiphertext::from_string(&record_org.record_ciphertext) {
                if let Ok(plaintext) = record.decrypt(&ViewKey::from_private_key(&self)) {
                    let program_id = record_org.program_id.clone();

                    let record_name = &record_org.identifier;
                    if let Ok(serial_number) = plaintext.serial_number_string(&self, &program_id, record_name) {
                        let record_data: RecordData<CurrentNetwork> = RecordData {
                            record: plaintext.deref().clone(),
                            identifier: record_org.identifier,
                            serial_number,
                            program_id,
                            height: record_org.height,
                            timestamp: record_org.timestamp,
                            block_hash: record_org.block_hash,
                            transaction_id: record_org.transaction_id,
                            transition_id: record_org.transition_id,
                            function_name: record_org.function_name,
                            output_index: record_org.output_index,
                            input: record_org.input,
                        };
                        records.push(record_data)
                    };
                };
            };
        }
        Ok(serde_json::to_string_pretty(&records).unwrap_or_default().replace("\\n", ""))
    }
}

impl From<PrivateKeyNative> for PrivateKey {
    fn from(private_key: PrivateKeyNative) -> Self {
        Self(private_key)
    }
}

impl From<PrivateKey> for PrivateKeyNative {
    fn from(private_key: PrivateKey) -> Self {
        private_key.0
    }
}

impl From<&PrivateKey> for PrivateKeyNative {
    fn from(private_key: &PrivateKey) -> Self {
        private_key.0
    }
}
impl FromStr for PrivateKey {
    type Err = anyhow::Error;

    fn from_str(private_key: &str) -> Result<Self, Self::Err> {
        Ok(Self(PrivateKeyNative::from_str(private_key)?))
    }
}

impl fmt::Display for PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for PrivateKey {
    type Target = PrivateKeyNative;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rand::Rng;
    use wasm_bindgen_test::*;

    const ITERATIONS: u64 = 1_000;

    const ALEO_PRIVATE_KEY: &str = "APrivateKey1zkp3dQx4WASWYQVWKkq14v3RoQDfY2kbLssUj7iifi1VUQ6";
    const ALEO_VIEW_KEY: &str = "AViewKey1cxguxtKkjYnT9XDza9yTvVMxt6Ckb1Pv4ck1hppMzmCB";
    const ALEO_ADDRESS: &str = "aleo184vuwr5u7u0ha5f5k44067dd2uaqewxx6pe5ltha5pv99wvhfqxqv339h4";

    #[wasm_bindgen_test]
    pub fn test_sanity_check() {
        let private_key = PrivateKey::from_string(ALEO_PRIVATE_KEY).unwrap();

        println!("{} == {}", ALEO_PRIVATE_KEY, private_key.to_string());
        assert_eq!(ALEO_PRIVATE_KEY, private_key.to_string());

        println!("{} == {}", ALEO_VIEW_KEY, private_key.to_view_key());
        assert_eq!(ALEO_VIEW_KEY, private_key.to_view_key().to_string());

        println!("{} == {}", ALEO_ADDRESS, private_key.to_address());
        assert_eq!(ALEO_ADDRESS, private_key.to_address().to_string());
    }

    #[wasm_bindgen_test]
    pub fn test_new() {
        for _ in 0..ITERATIONS {
            // Generate a new private_key.
            let expected = PrivateKey::new();

            // Check the private_key derived from string.
            assert_eq!(expected, PrivateKey::from_string(&expected.to_string()).unwrap());
        }
    }

    #[wasm_bindgen_test]
    pub fn test_from_seed_unchecked() {
        for _ in 0..ITERATIONS {
            // Sample a random seed.
            let seed: [u8; 32] = StdRng::from_entropy().gen();

            // Ensure the private key is deterministically recoverable.
            let expected = PrivateKey::from_seed_unchecked(&seed);
            assert_eq!(expected, PrivateKey::from_seed_unchecked(&seed));
        }
    }

    #[wasm_bindgen_test]
    pub fn test_to_address() {
        for _ in 0..ITERATIONS {
            // Sample a new private key.
            let private_key = PrivateKey::new();
            let expected = private_key.to_address();

            // Check the private_key derived from the view key.
            let view_key = private_key.to_view_key();
            assert_eq!(expected, Address::from_view_key(&view_key));
        }
    }

    #[wasm_bindgen_test]
    pub fn test_signature() {
        for _ in 0..ITERATIONS {
            // Sample a new private key and message.
            let private_key = PrivateKey::new();
            let message: [u8; 32] = StdRng::from_entropy().gen();

            // Sign the message.
            let signature = private_key.sign(&message);
            // Check the signature is valid.
            assert!(signature.verify(&private_key.to_address(), &message));
            // Check the signature is valid (natively).
            assert!(signature.verify_bytes(&private_key.to_address(), &message));
        }
    }

    #[wasm_bindgen_test]
    fn test_private_key_ciphertext_encrypt_and_decrypt() {
        let private_key = PrivateKey::new();
        let private_key_ciphertext = PrivateKeyCiphertext::encrypt_private_key(&private_key, "mypassword").unwrap();
        let recovered_private_key = private_key_ciphertext.decrypt_to_private_key("mypassword").unwrap();
        assert_eq!(private_key, recovered_private_key);
    }
}
