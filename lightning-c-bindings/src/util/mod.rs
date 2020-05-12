//! " Some utility modules live here. See individual sub-modules for more info."

use std::ffi::c_void;
use bitcoin::hashes::Hash;
use crate::c_types::TakePointer;

pub mod events;
pub mod errors;
pub mod logger;
pub mod config;
