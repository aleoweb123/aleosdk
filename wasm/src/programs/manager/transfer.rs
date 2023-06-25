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

use super::*;

use crate::{
    execute_program,
    get_process,
    log,
    types::{CurrentAleo, CurrentNetwork, CurrentBlockMemory, IdentifierNative, ProcessNative, ProgramNative, TransactionNative, ConsensusStoreNative, ValueNative, ConsensusMemoryNative, RecordPlaintextNative, VMNative},
    PrivateKey,
    RecordPlaintext,
    Transaction,
};

use js_sys::Array;
use lazy_static::__Deref;
use rand::{rngs::StdRng, SeedableRng};
use std::{ops::Add, str::FromStr};

#[wasm_bindgen]
impl ProgramManager {
    /// Send credits from one Aleo account to another
    ///
    /// @param private_key The private key of the sender
    /// @param amount_credits The amount of credits to send
    /// @param recipient The recipient of the transaction
    /// @param transfer_type The type of the transfer (options: "private", "public", "private_to_public", "public_to_private")
    /// @param amount_record The record to fund the amount from
    /// @param fee_credits The amount of credits to pay as a fee
    /// @param fee_record The record to spend the fee from
    /// @param url The url of the Aleo network node to send the transaction to
    /// @param cache Cache the proving and verifying keys in the ProgramManager memory. If this is
    /// set to `true` the keys synthesized (or passed in as optional parameters via the
    /// `transfer_proving_key` and `transfer_verifying_key` arguments) will be stored in the
    /// ProgramManager's memory and used for subsequent transactions. If this is set to `false` the
    /// proving and verifying keys will be deallocated from memory after the transaction is executed
    /// @param transfer_proving_key (optional) Provide a proving key to use for the transfer
    /// function
    /// @param transfer_verifying_key (optional) Provide a verifying key to use for the transfer
    /// function
    /// @param fee_proving_key (optional) Provide a proving key to use for the fee execution
    /// @param fee_verifying_key (optional) Provide a verifying key to use for the fee execution
    #[wasm_bindgen]
    #[allow(clippy::too_many_arguments)]
    pub async fn transfer(
        &mut self,
        private_key: PrivateKey,
        amount_credits: f64,
        recipient: String,
        transfer_type: String,
        amount_record: Option<RecordPlaintext>,
        fee_credits: f64,
        fee_record: RecordPlaintext,
        url: String,
        cache: bool,
        transfer_proving_key: Option<ProvingKey>,
        transfer_verifying_key: Option<VerifyingKey>,
        fee_proving_key: Option<ProvingKey>,
        fee_verifying_key: Option<VerifyingKey>,
    ) -> Result<Transaction, String> {
        log("Executing transfer program");
        let amount_microcredits = if let Some(amount_record) = amount_record.as_ref() {
            Self::validate_amount(amount_credits, amount_record, false)?
        } else {
            (amount_credits * 1_000_000.0) as u64
        };
        let fee_microcredits = Self::validate_amount(fee_credits, &fee_record, true)?;

        log("Setup the program and inputs");
        let program = ProgramNative::credits().unwrap().to_string();
        let inputs = Array::new_with_length(3);
        log(&format!("transfer program {}, transfer_type {}", program, transfer_type));

        let transfer_type = match transfer_type.as_str() {
            "private" => "transfer_".to_string().add("private"),
            "private_to_public" => "transfer_".to_string().add("private_to_public"),
            "public" => "transfer_".to_string().add("public"),
            "public_to_private" => "transfer_".to_string().add("public_to_private"),
            _ => transfer_type,
        };
        log(&format!("transfer transfer_type {}", transfer_type));

        let transfer_type = match transfer_type.as_str() {
            "transfer_private" => {
                if amount_record.is_none() {
                    return Err("Amount record must be provided for private transfers".to_string());
                }
                inputs.set(0u32, wasm_bindgen::JsValue::from_str(&amount_record.unwrap().to_string()));
                inputs.set(1u32, wasm_bindgen::JsValue::from_str(&recipient));
                inputs.set(2u32, wasm_bindgen::JsValue::from_str(&amount_microcredits.to_string().add("u64")));
                transfer_type
            }
            "transfer_private_to_public" => {
                if amount_record.is_none() {
                    return Err("Amount record must be provided for private transfers".to_string());
                }
                inputs.set(0u32, wasm_bindgen::JsValue::from_str(&amount_record.unwrap().to_string()));
                inputs.set(1u32, wasm_bindgen::JsValue::from_str(&recipient));
                inputs.set(2u32, wasm_bindgen::JsValue::from_str(&amount_microcredits.to_string().add("u64")));
                transfer_type
            }
            "transfer_public" => {
                inputs.set(0u32, wasm_bindgen::JsValue::from_str(&recipient));
                inputs.set(1u32, wasm_bindgen::JsValue::from_str(&amount_microcredits.to_string().add("u64")));
                transfer_type
            }
            "transfer_public_to_private" => {
                inputs.set(1u32, wasm_bindgen::JsValue::from_str(&recipient));
                inputs.set(2u32, wasm_bindgen::JsValue::from_str(&amount_microcredits.to_string().add("u64")));
                transfer_type
            }
            _ => return Err("Invalid transfer type".to_string()),
        };

        let mut new_process;
        let process = get_process!(self, cache, new_process);
        log("transfer fee_identifier");
        let fee_identifier = IdentifierNative::from_str("fee").map_err(|e| e.to_string())?;
        log("transfer process get_stack");
        let stack = process.get_stack("credits.aleo").map_err(|e| e.to_string())?;
        if !stack.contains_proving_key(&fee_identifier) && fee_proving_key.is_some() && fee_verifying_key.is_some() {
            let fee_proving_key = fee_proving_key.unwrap();
            let fee_verifying_key = fee_verifying_key.unwrap();
            log("transfer stack insert_proving_key");
            stack
                .insert_proving_key(&fee_identifier, ProvingKeyNative::from(fee_proving_key))
                .map_err(|e| e.to_string())?;
            log("transfer stack insert_verifying_key");
            stack
                .insert_verifying_key(&fee_identifier, VerifyingKeyNative::from(fee_verifying_key))
                .map_err(|e| e.to_string())?;
        }

        log("transfer execute_program");
        let (_, mut trace) = execute_program!(
            process,
            inputs,
            program,
            &transfer_type,
            private_key,
            transfer_proving_key,
            transfer_verifying_key
        );

        log("transfer trace prepare_async");
        // Prepare the inclusion proofs for the fee & execution
        trace.prepare_async::<CurrentBlockMemory, _>(&url).await.map_err(|err| err.to_string())?;

        let program =
        ProgramNative::from_str(&program).map_err(|_| "The program ID provided was invalid".to_string())?;

        let locator = program.id().to_string().add("/").add(&transfer_type);
        log(&format!("transfer trace prove_execution locator {locator}"));
        // Prove the execution and fee
        let execution = trace
            .prove_execution::<CurrentAleo, _>(&locator, &mut StdRng::from_entropy())
            .map_err(|e| e.to_string())?;

        log("transfer trace prove_fee");

        log("Executing fee program");
        log("transfer execution to_execution_id");
        let execution_id = execution.to_execution_id().map_err(|e| e.to_string())?;

        let fee_record_native = RecordPlaintextNative::from_str(&fee_record.to_string()).unwrap();
        let (_, _, trace) = process
            .execute_fee::<CurrentAleo, _>(
                &private_key,
                fee_record_native,
                fee_microcredits,
                execution_id,
                &mut StdRng::from_entropy(),
            )
            .map_err(|err| err.to_string())?;

        let fee = trace.prove_fee::<CurrentAleo, _>(&mut StdRng::from_entropy()).map_err(|e| e.to_string())?;
        

        // Verify the execution and fee
        log("transfer process verify_execution");
        process.verify_execution(&execution).map_err(|err| err.to_string())?;
        log("transfer process verify_fee");
        process.verify_fee(&fee, execution_id).map_err(|err| err.to_string())?;

        log("Creating execution transaction for transfer");
        let transaction = TransactionNative::from_execution(execution, Some(fee)).map_err(|err| err.to_string())?;
        Ok(Transaction::from(transaction))
    }

    // #[wasm_bindgen(js_name = "newtransfer")]
    // #[allow(clippy::too_many_arguments)]
    // pub async fn new_transfer(
    //     &mut self,
    //     private_key: PrivateKey,
    //     amount_credits: f64,
    //     recipient: String,
    //     transfer_type: String,
    //     amount_record: Option<RecordPlaintext>,
    //     fee_credits: f64,
    //     fee_record: RecordPlaintext,
    //     url: String,
    //     cache: bool,
    //     transfer_proving_key: Option<ProvingKey>,
    //     transfer_verifying_key: Option<VerifyingKey>,
    //     fee_proving_key: Option<ProvingKey>,
    //     fee_verifying_key: Option<VerifyingKey>,
    // ) -> Result<Transaction, String> {
    //     log("newtransfer execution");
    //     let execution = {
    //         // Initialize an RNG.
    //         let rng = &mut rand::thread_rng();

    //         // Initialize the VM.
    //         log("newtransfer Initialize the VM");
    //         let store = ConsensusStoreNative::open(None).map_err(|e| e.to_string())?;
    //         let vm = VMNative::from(store).map_err(|e| e.to_string())?;

    //         let amount_microcredits = (amount_credits * 1_000_000.0) as u64;
    //         let amount_record = RecordPlaintextNative::from_str(&amount_record.unwrap().to_string()).map_err(|e| e.to_string())?;
    //         let fee = (fee_credits * 1_000_000.0) as u64;
    //         let fee = (RecordPlaintextNative::from_str(&fee_record.to_string()).map_err(|e| e.to_string())?, fee);

    //         let transfer_type = match transfer_type.as_str() {
    //             "private" => "transfer_".to_string().add("private"),
    //             "private_to_public" => "transfer_".to_string().add("private_to_public"),
    //             "public" => "transfer_".to_string().add("public"),
    //             "public_to_private" => "transfer_".to_string().add("public_to_private"),
    //             _ => transfer_type,
    //         };
    //         log(&format!("transfer transfer_type {}", transfer_type));
    //         // Prepare the inputs for a transfer.
            
    //         // let inputs = Array::new_with_length(3);
            
    //         // let transfer_type = match transfer_type.as_str() {
    //         //     "transfer_private" => {
    //         //         if amount_record.is_none() {
    //         //             return Err("Amount record must be provided for private transfers".to_string());
    //         //         }
    //         //         inputs.set(0u32, wasm_bindgen::JsValue::from_str(&amount_record.unwrap().to_string()));
    //         //         inputs.set(1u32, wasm_bindgen::JsValue::from_str(&recipient));
    //         //         inputs.set(2u32, wasm_bindgen::JsValue::from_str(&amount_microcredits.to_string().add("u64")));
    //         //         transfer_type
    //         //     }
    //         //     "transfer_private_to_public" => {
    //         //         if amount_record.is_none() {
    //         //             return Err("Amount record must be provided for private transfers".to_string());
    //         //         }
    //         //         inputs.set(0u32, wasm_bindgen::JsValue::from_str(&amount_record.unwrap().to_string()));
    //         //         inputs.set(1u32, wasm_bindgen::JsValue::from_str(&recipient));
    //         //         inputs.set(2u32, wasm_bindgen::JsValue::from_str(&amount_microcredits.to_string().add("u64")));
    //         //         transfer_type
    //         //     }
    //         //     "transfer_public" => {
    //         //         inputs.set(0u32, wasm_bindgen::JsValue::from_str(&recipient));
    //         //         inputs.set(1u32, wasm_bindgen::JsValue::from_str(&amount_microcredits.to_string().add("u64")));
    //         //         transfer_type
    //         //     }
    //         //     "transfer_public_to_private" => {
    //         //         inputs.set(1u32, wasm_bindgen::JsValue::from_str(&recipient));
    //         //         inputs.set(2u32, wasm_bindgen::JsValue::from_str(&amount_microcredits.to_string().add("u64")));
    //         //         transfer_type
    //         //     }
    //         //     _ => return Err("Invalid transfer type".to_string()),
    //         // };

    //         let inputs = vec![
    //             ValueNative::Record(amount_record),
    //             ValueNative::from_str(&format!("{}", recipient)).map_err(|e| e.to_string())?,
    //             ValueNative::from_str(&format!("{}u64", amount_microcredits)).map_err(|e| e.to_string())?,
    //         ];
    //         log("newtransfer Create a new transaction.");
    //         // Create a new transaction.
    //         vm.execute(&private_key, ("credits.aleo", "transfer_private"), inputs.iter(), Some(fee), None, rng).map_err(|e| e.to_string())?
    //     };
    //     Ok(Transaction::from(execution))
    // }
}
