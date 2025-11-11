//! Middleware module group for the backend.
pub mod auth;
pub mod error_handler;
pub mod rate_limit;

// Re-export commonly used middleware pieces for convenience.
pub use auth::*;
pub use error_handler::*;
pub use rate_limit::*;
