// villin_REIGN - Military-grade IoT security

pub mod core;
"""
    - Declares and makes the core module public
    - Looks for either src/core/mod.rs or src/core.rs
    - 'pub' meanss ither code can access this module
"""
pub mod security;
"""
    - Declares and makes the security public
    - Look for src/security/mod.rs or src/security.rs
"""
pub mod api;
"""
    - Declares and makes tge api module public
    - Looks for src/api/mod.rs or src/api.rs
"""
pub mod monitoring;
"""
    - Declares and makes tthe monitoring module public
    - Looks for src/monitoring/mod.rs or src/monitoring
"""
pub use core::REIGN



// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION")