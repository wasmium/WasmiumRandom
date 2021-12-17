#![deny(missing_docs)]

//! This crate uses the [nanorand](docs.rs/nanorand) crate to generate random bytes
//! from the English alphabet characters, Alphanumeric characters and numeric characters.
//!
//! This crate does not allocate on the heap. Any heap allocation is a bug, please file an issue on Github.
//! Parallelism is planned at a later date.
//!
//! ### Usage
//! Import the crate
//! ```
//! use wasmium_random::WasmiumRandom;
//! ```
//!
//! ### Generate bytes securely from a CSPRNG using the alphabet `a-z,A-Z`. This is useful for cryptographic applications.
//! ```rust
//! # use wasmium_random::WasmiumRandom;
//!
//! // Generate 12 random bytes
//! WasmiumRandom::secure_alphabet12();
//!
//! // Generate 24 random bytes
//! WasmiumRandom::secure_alphabet24();
//!
//! // Generate 32 random bytes
//! WasmiumRandom::secure_alphabet32();
//!
//! // Generate 64 random bytes
//! WasmiumRandom::secure_alphabet64();
//!
//! // Generate 128 random bytes
//! WasmiumRandom::secure_alphabet128();
//!
//! // Generate 256 random bytes
//! WasmiumRandom::secure_alphabet256();
//!
//! // Generate 512 random bytes
//! WasmiumRandom::secure_alphabet512();
//! ```
//!
//! ### Generate bytes securely from a CSPRNG using the alphanumeric characters `0-9,a-z,A-Z`. This is useful for cryptographic applications.
//! ```rust
//! # use wasmium_random::WasmiumRandom;
//!
//! // Generate 12 random bytes securely
//! WasmiumRandom::secure_alphanumeric12();
//!
//! // Generate 24 random bytes securely
//! WasmiumRandom::secure_alphanumeric24();
//!
//! // Generate 32 random bytes securely
//! WasmiumRandom::secure_alphanumeric32();
//!
//! // Generate 64 random bytes securely
//! WasmiumRandom::secure_alphanumeric64();
//!
//! // Generate 128 random bytes securely
//! WasmiumRandom::secure_alphanumeric128();
//!
//! // Generate 512 random bytes securely
//! WasmiumRandom::secure_alphanumeric512();
//! ```
//!
//! ### Generate bytes securely from a CSPRNG using the numeric characters `0-9`. This is useful for cryptographic applications.
//! ```rust
//! # use wasmium_random::WasmiumRandom;
//!
//! // Generate 12 random bytes securely
//! WasmiumRandom::secure_numeric12();
//!
//! // Generate 24 random bytes securely
//! WasmiumRandom::secure_numeric24();
//!
//! // Generate 32 random bytes securely
//! WasmiumRandom::secure_numeric32();
//!
//! // Generate 64 random bytes securely
//! WasmiumRandom::secure_numeric64();
//!
//! // Generate 128 random bytes securely
//! WasmiumRandom::secure_numeric128();
//!
//! // Generate 512 random bytes securely
//! WasmiumRandom::secure_numeric512();
//! ```
//!
//!
//! ## Non-cryptographically secure random bytes are also supported using the super fast [Wyrand](nanorand::WyRand) PRNG.
//! ### Generate bytes from a PRNG using the alphabet `a-z,A-Z`.
//! ```rust
//! # use wasmium_random::WasmiumRandom;
//!
//! // Generate 12 random bytes
//! WasmiumRandom::wyrand_alphabet12();
//!
//! // Generate 24 random bytes
//! WasmiumRandom::wyrand_alphabet24();
//!
//! // Generate 32 random bytes
//! WasmiumRandom::secure_alphabet32();
//!
//! // Generate 64 random bytes
//! WasmiumRandom::wyrand_alphabet64();
//!
//! // Generate 128 random bytes
//! WasmiumRandom::wyrand_alphabet128();
//!
//! // Generate 256 random bytes
//! WasmiumRandom::wyrand_alphabet256();
//!
//! // Generate 512 random bytes
//! WasmiumRandom::wyrand_alphabet512();
//! ```
//!
//! ### Generate bytes from a PRNG using the alphanumeric characters `0-9,a-z,A-Z`.
//! ```rust
//! # use wasmium_random::WasmiumRandom;
//!
//! // Generate 12 random bytes
//! WasmiumRandom::wyrand_alphanumeric12();
//!
//! // Generate 24 random bytes
//! WasmiumRandom::wyrand_alphanumeric24();
//!
//! // Generate 32 random bytes
//! WasmiumRandom::wyrand_alphanumeric32();
//!
//! // Generate 64 random bytes
//! WasmiumRandom::wyrand_alphanumeric64();
//!
//! // Generate 128 random bytes
//! WasmiumRandom::wyrand_alphanumeric128();
//!
//! // Generate 512 random bytes
//! WasmiumRandom::wyrand_alphanumeric512();
//! ```
//!
//! ### Generate bytes from a PRNG using the numeric characters `0-9`.
//! ```
//! # use wasmium_random::WasmiumRandom;
//!
//! // Generate 12 random bytes securely
//! WasmiumRandom::wyrand_numeric12();
//!
//! // Generate 24 random bytes securely
//! WasmiumRandom::wyrand_numeric24();
//!
//! // Generate 32 random bytes securely
//! WasmiumRandom::wyrand_numeric32();
//!
//! // Generate 64 random bytes securely
//! WasmiumRandom::wyrand_numeric64();
//!
//! // Generate 128 random bytes securely
//! WasmiumRandom::wyrand_numeric128();
//!
//! // Generate 512 random bytes securely
//! WasmiumRandom::wyrand_numeric512();
//!
//! ```
//! `NOTE:` The bytes from characters can be generated to upto 2048 bytes. See the submodule
//! documentation.
//!
//! The BIP39, EFF shortlist and EFF largelist of English alphabet words are also supported.
//!
mod common;
pub use common::*;

mod csprng;
pub use csprng::*;

mod prng;
pub use prng::*;

mod wordlists;
pub use wordlists::*;
