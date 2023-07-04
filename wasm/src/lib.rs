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

//!
//! [![Crates.io](https://img.shields.io/crates/v/aleo-wasm.svg?color=neon)](https://crates.io/crates/aleo-wasm)
//! [![Authors](https://img.shields.io/badge/authors-Aleo-orange.svg)](https://aleo.org)
//! [![License](https://img.shields.io/badge/License-GPLv3-blue.svg)](./LICENSE.md)
//!
//! [![github]](https://github.com/AleoHQ/sdk)&ensp;[![crates-io]](https://crates.io/crates/aleo-wasm)&ensp;[![docs-rs]](https://docs.rs/aleo-wasm/latest/aleo-wasm/)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! # Aleo Wasm
//!
//! Aleo JavaScript and WebAssembly bindings for building zero-knowledge web applications.
//!
//! `Rust` compiles easily to `WebAssembly` but creating the glue code necessary to use compiled WebAssembly binaries
//! from other languages such as JavaScript is a challenging task. `wasm-bindgen` is a tool that simplifies this process by
//! auto-generating JavaScript bindings to Rust code that has been compiled into WebAssembly.
//!
//! This crate uses `wasm-bindgen` to create JavaScript bindings to Aleo source code so that it can be used to create zero
//! knowledge proofs directly within `web browsers` and `NodeJS`.
//!
//! Functionality exposed by this crate includes:
//! * Aleo account management objects
//! * Aleo primitives such as `Records`, `Programs`, and `Transactions` and their associated helper methods
//! * A `ProgramManager` object that contains methods for authoring, deploying, and interacting with Aleo programs
//!
//! More information on these concepts can be found at the [Aleo Developer Hub](https://developer.aleo.org/concepts).
//!
//! ## Usage
//! The [wasm-pack](https://crates.io/crates/wasm-pack) tool is used to compile the Rust code in this crate into JavaScript
//! modules which can be imported into other JavaScript projects.
//!
//! #### Install Wasm-Pack
//! ```bash
//! curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
//! ```
//!
//! ### Build Instructions
//! The general syntax for compiling rust into WebAssembly based JavaScript modules with
//! [wasm-pack](https://crates.io/crates/wasm-pack) is as follows:
//! ```bash
//! wasm-pack build --target <target> --out-dir <out-dir> -- --features <crate-features>
//! ```
//!
//! Invoking this command will build a JavaScript module in the current directory with the default name `pkg` (which can
//! be changed as necessary using the `--out-dir` flag). This folder can then be imported directly as a JavaScript module
//! by other JavaScript modules.
//!
//! There are 3 possible JavaScript modules that [wasm-pack](https://crates.io/crates/wasm-pack) can be used to generate
//! when run within this crate:
//! 1. **NodeJS module:** Used to build NodeJS applications.
//! 2. **Single-Threaded browser module:** Used to build browser-based web applications.
//! 3. **Multi-Threaded browser module:** Used to build browser-based web applications which use web-worker based
//! multi-threading to achieve significant performance increases.
//!
//! These 3 modules and how to build them are explained in more detail below.
//!
//! ### 1. NodeJS Module
//!
//! This module has the features of the NodeJS environment built-in. It is single-threaded and unfortunately cannot yet be
//! used to generate Aleo program executions or deployments due to current Aleo protocol limitations. It can however still
//! be used to perform Aleo account, record, and program management tasks.
//!
//! #### Build Instructions
//! ```bash
//! wasm-pack build --release --target nodejs -- --features "serial" --no-default-features
//! ```
//!
//! ### 2. Single-Threaded browser module
//!
//! This module is very similar to the NodeJS module, however it is built to make use browser-based JavaScript environments
//! and can be used for program execution and deployment.
//!
//! If used for program execution or deployment, it suggested to do so on a web-worker as these operations are long-running
//! and will cause a browser window to hang if run in the main thread.
//!
//! #### Build Instructions
//! ```bash
//! wasm-pack build --release --target web
//! ```
//!
//! If you are intending to use this for program execution or deployment, it is recommended to build
//! with maximum or close to maximum memory allocation (which is 4 gigabytes for wasm).
//!
//! ```bash
//! RUSTFLAGS='-C link-arg=--max-memory=4294967296' wasm-pack build --release --target web
//! ````
//!
//! ### 3. Multi-Threaded browser module
//!
//! This module is also built for browser-based JavaScript environments, however it is built to make use of Rust-native
//! threading via web-workers (using the approach outlined in the `rayon-wasm-bindgen` crate). It is the most complex to use,
//! but it will run significantly faster when performing Aleo program executions and deployments and should be the choice for
//! performance-critical applications.
//!
//! To build with threading enabled, it is necessary to use `nightly Rust` and set certain `RUSTFLAGS` to enable the
//! necessary threading features. The `wasm-pack` build command is shown below.
//! ```bash
//! # Set rustflags to enable atomics,
//! # bulk-memory, and mutable-globals.
//! # Also, set the maximum memory to
//! # 4294967296 bytes (4GB).
//! export RUSTFLAGS='-C target-feature=+atomics,+bulk-memory,+mutable-globals -C link-arg=--max-memory=4294967296'
//!
//! # Use rustup to run the following commands
//! # with the nightly version of Rust.
//! rustup run nightly \
//!
//! # Use wasm-pack to build the project.
//! # Specify the 'parallel' feature for
//! # multi-threading and the 'browser'
//! # feature to enable program execution
//! # and include necessary unstable options
//! # using -Z
//! wasm-pack build --release --target web --out-dir pkg-parallel \
//! -- --features "parallel, browser" -Z build-std=panic_abort,std
//! ```
//!
//! ## Testing
//!
//! Run tests in NodeJS
//! ```bash
//! wasm-pack test --node
//! ```
//!
//! Run tests in a browser
//! ```bash
//! wasm-pack test --[firefox/chrome/safari]
//! ```
//!
//! ## Building Web Apps
//!
//! Further documentation and tutorials as to how to use the modules built from this crate to build web apps  will be built
//! in the future. However - in the meantime, the [aleo.tools](https://aleo.tools) website is a good
//! example of how to use these modules to build a web app. Its source code can be found in the
//!

pub mod account;
pub use account::*;

pub mod programs;
pub use programs::*;

pub mod record;
pub use record::*;

pub(crate) mod types;

use wasm_bindgen::prelude::*;
use num_bigint::BigUint;

#[cfg(feature = "parallel")]
pub use wasm_bindgen_rayon::init_thread_pool;

// use crate::types::{AValueNative, ALiteralNative, ProgramNative, ValueNative, LiteralNative,FieldlNative, LiteralTypeNative};

use aleo_rust::{Field, Literal, Testnet3, Value};
use snarkvm_circuit_program::{Literal as ALiteral, Value as AValue};
use snarkvm_console::{
    prelude::{ToField, TypeName},
    program::LiteralType,
};
use snarkvm_synthesizer::output_type;
use std::{ops::Deref, str::FromStr, string};

use aleo_rust::ToBytes;
use snarkvm_circuit_environment::{Eject, Inject, Mode, ToBits as AToBits};
use snarkvm_circuit_network::{Aleo, AleoV0};
use snarkvm_wasm::FromBytes;

// Facilities for cross-platform logging in both web browsers and nodeJS
#[wasm_bindgen]
extern "C" {
    // Log a &str the console in the browser or console.log in nodejs
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(js_name = "base58")]
pub fn Base58(input: &str, action: &str) -> Result<String, String> {
    match action {
        "encode" => {
            let bytes = bs58::encode(input.as_bytes().to_vec()).into_vec();
            let encodecode = bs58::encode(input.as_bytes().to_vec()).into_string();

            let big_int = BigUint::from_bytes_be(&bytes);
            let big_int_str = big_int.to_string();
            log(&format!("{} {}", encodecode, big_int_str));
            Ok(format!("{}{}", big_int_str, Field::<Testnet3>::type_name()))
        }
        "decode" => {
            let input = if input.ends_with("field") { &input[..input.len() - "field".len()] } else { input };
            if let Some(bitint) = BigUint::parse_bytes(input.as_bytes(), 10) {
                let bytes = bitint.to_bytes_be();
                let encodecode = String::from_utf8(bytes.clone()).map_err(|e| format!("invalid encodecode from_utf8: {e}"))?;
                let bytes = bs58::decode(&encodecode).into_vec().map_err(|e| format!("invalid decode encodecode: {e}"))?;
                let decodecode = String::from_utf8(bytes.clone()).map_err(|e| format!("invalid decodecode from_utf8: {e}"))?;
                return Ok(decodecode);
            };
            Err("BigUint parse_bytes err".to_string())
        }
        &_ => Err("Invalid base58 action ,use (encode or decode)".to_string()),
    }
}

#[wasm_bindgen(js_name = "hashBHP")]
pub fn hash_bhp(input: String, bhptype: &str, destination_type: &str) -> Result<String, String> {
    let value = Value::<Testnet3>::from_str(&input).map_err(|e| format!("invalid input: {e}"))?;
    let avalue = AValue::<AleoV0>::new(Mode::Public, value.clone());
    let destination_type =
        LiteralType::from_str(destination_type).map_err(|e| format!("invalid destination type: {e}"))?;
    let output_type = match bhptype {
        "bhp256" => ALiteral::Group(Aleo::hash_to_group_bhp256(&avalue.to_bits_le())),
        "bhp512" => ALiteral::Group(Aleo::hash_to_group_bhp512(&avalue.to_bits_le())),
        "bhp768" => ALiteral::Group(Aleo::hash_to_group_bhp768(&avalue.to_bits_le())),
        "bhp1024" => ALiteral::Group(Aleo::hash_to_group_bhp1024(&avalue.to_bits_le())),
        _ => return Err("Invalid bhptype type".to_string()),
    };
    let output = output_type.downcast_lossy(destination_type).map_err(|e| format!("failed to downcast: {e}"))?;

    let fieldbytes = literal_to_bytes(output.eject_value()).map_err(|e| format!("literal_to_bytes: {e}"))?;

    let field = Field::<Testnet3>::from_bytes_le(&fieldbytes).map_err(|e| format!("invalid fieldbytes: {e}"))?;
    Ok(format!("{}{}", field, Field::<Testnet3>::type_name()))
}

fn literal_to_bytes(literal: Literal<Testnet3>) -> anyhow::Result<Vec<u8>> {
    match literal {
        Literal::Address(v) => v.to_bytes_le(),
        Literal::Boolean(v) => v.to_bytes_le(),
        Literal::Field(v) => v.to_bytes_le(),
        Literal::Group(v) => v.to_bytes_le(),
        Literal::I8(v) => v.to_bytes_le(),
        Literal::I16(v) => v.to_bytes_le(),
        Literal::I32(v) => v.to_bytes_le(),
        Literal::I64(v) => v.to_bytes_le(),
        Literal::I128(v) => v.to_bytes_le(),
        Literal::U8(v) => v.to_bytes_le(),
        Literal::U16(v) => v.to_bytes_le(),
        Literal::U32(v) => v.to_bytes_le(),
        Literal::U64(v) => v.to_bytes_le(),
        Literal::U128(v) => v.to_bytes_le(),
        Literal::Scalar(v) => v.to_bytes_le(),
        Literal::String(v) => v.to_bytes_le(),
    }
}
