//! " Structs and traits which allow other parts of rust-lightning to interact with the blockchain."

use std::ffi::c_void;
use bitcoin::hashes::Hash;

pub mod chaininterface;
pub mod transaction;
pub mod keysinterface;
