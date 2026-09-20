//! FinFlowRL — Rust configuration subsystem.
//!
//! The crate currently ports the configuration system from
//! `src/finflowrl/config/settings.py` (see `config.rs`). The simulator,
//! environment, policy, and training loops are the Python implementation
//! under `src/finflowrl/`; they are deterministic research/accounting
//! loops rather than hot paths, so a full Rust port is deliberately out
//! of scope (maintenance risk with no user-visible benefit).

pub mod config;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
