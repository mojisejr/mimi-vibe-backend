//! Middleware module group for the backend.
pub mod auth;
pub mod rate_limit;
pub mod error_handler;

// Re-export commonly used middleware pieces for convenience.
pub use auth::*;
pub use rate_limit::*;
pub use error_handler::*;
