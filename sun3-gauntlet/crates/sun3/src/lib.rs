//! Sun Microsystems Sun 3/60 machine emulation.

pub mod idprom;
pub mod machine;
pub mod scc;

pub use machine::{MachineConfig, RunOutcome, Sun3Machine, TraceFlags};
