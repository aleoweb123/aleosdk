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
    log,
    types::{
        CurrentAleo,
        CurrentNetwork,
        ProcessNative,
        ProgramNative,
    },
    PrivateKey,
    execute_program, process_inputs,
};

use js_sys::{Object, Array};
use rand::{rngs::StdRng, SeedableRng};
use std::{str::FromStr, ops::Add};

#[wasm_bindgen]
impl ProgramManager {
    #[wasm_bindgen(js_name = costDeployment)]
    #[allow(clippy::too_many_arguments)]
    pub async fn deployment_cost(program: &str, imports: Option<Object>) -> Result<String, String> {
        log("Creating deployment transaction");
        let mut process_native = ProcessNative::load_web().map_err(|err| err.to_string())?;
        let process = &mut process_native;

        log("Checking program has a valid name");
        let program = ProgramNative::from_str(program).map_err(|err| err.to_string())?;

        log("Checking program imports are valid and add them to the process");
        ProgramManager::resolve_imports(process, &program, imports)?;
        let rng = &mut StdRng::from_entropy();

        log("Creating deployment");
        let deployment = process.deploy::<CurrentAleo, _>(&program, rng).map_err(|err| err.to_string())?;
        if deployment.program().functions().is_empty() {
            return Err("Attempted to create an empty transaction deployment".to_string());
        }

        log("Ensuring the fee is sufficient to pay for the deployment");
        let (minimum_deployment_cost, (storage_cost, finalize_cost)) =
            deployment_cost::<CurrentNetwork>(&deployment).map_err(|err| err.to_string())?;
    
        let json_object = serde_json::json!({
            "minimum_deployment_cost":minimum_deployment_cost,
            "storage_cost":storage_cost,
            "finalize_cost":finalize_cost,
        });
        
        Ok(json_object.to_string())
    }

    #[wasm_bindgen(js_name = costExecution)]
    #[allow(clippy::too_many_arguments)]
    pub async fn execution_cost(
        private_key: &PrivateKey,
        program: &str,
        function: &str,
        inputs: Array,
        url: &str,
        imports: Option<Object>,
        proving_key: Option<ProvingKey>,
        verifying_key: Option<VerifyingKey>,
    ) -> Result<String, String> {
        let mut process_native = ProcessNative::load_web().map_err(|err| err.to_string())?;
        let process = &mut process_native;

        log("Check program imports are valid and add them to the process");
        let program_native = ProgramNative::from_str(program).map_err(|e| e.to_string())?;
        ProgramManager::resolve_imports(process, &program_native, imports)?;
        let rng = &mut StdRng::from_entropy();

        log("Executing program");
        let (_, mut trace) = execute_program!(
            process,
            process_inputs!(inputs),
            program,
            function,
            private_key,
            proving_key,
            verifying_key,
            rng
        );

        log("Preparing inclusion proofs for execution");
        let query = QueryNative::from(url);
        trace.prepare_async(query).await.map_err(|err| err.to_string())?;

        log("Proving execution");
        let program = ProgramNative::from_str(program).map_err(|err| err.to_string())?;
        let locator = program.id().to_string().add("/").add(function);
        let execution = trace
            .prove_execution::<CurrentAleo, _>(&locator, &mut StdRng::from_entropy())
            .map_err(|e| e.to_string())?;

        // Get the storage cost in bytes for the program execution
        let storage_cost = execution.size_in_bytes().map_err(|e| e.to_string())?;

        // Compute the finalize cost in microcredits.
        let mut finalize_cost = 0u64;
        // Iterate over the transitions to accumulate the finalize cost.
        for transition in execution.transitions() {
            // Retrieve the function name.
            let function_name = transition.function_name();
            // Retrieve the finalize cost.
            let cost = match program.get_function(function_name).map_err(|e| e.to_string())?.finalize_logic() {
                Some(finalize) => cost_in_microcredits(finalize).map_err(|e| e.to_string())?,
                None => continue,
            };
            // Accumulate the finalize cost.
            finalize_cost = finalize_cost
                .checked_add(cost)
                .ok_or("The finalize cost computation overflowed for an execution".to_string())?;
        }
        let minimum_fee_cost = finalize_cost + storage_cost;
        let json_object = serde_json::json!({
            "minimum_execution_cost":minimum_fee_cost,
            "storage_cost":storage_cost,
            "finalize_cost":finalize_cost,
        });
        
        Ok(json_object.to_string())
    }
}