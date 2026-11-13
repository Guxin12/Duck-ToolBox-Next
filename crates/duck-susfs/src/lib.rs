//! SuSFS manager backend (BRENE-compatible behaviour).
//!
//! Exposes `duckd susfs ...` with the same toggles and custom lists as upstream BRENE.
//! Shell scripts are replaced by this crate; the `ksu_susfs` / `susfs` binary is still used.

mod apply;
mod cli;
mod config;
mod custom;
mod error;
mod paths;
mod status;
mod susfs;

pub use cli::{Command, FEATURE};
pub use config::SusfsConfig;
pub use error::{code_of, fail};
