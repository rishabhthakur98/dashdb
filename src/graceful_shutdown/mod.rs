#[cfg(feature = "local-env")]
pub mod graceful_shutdown_dev;

#[cfg(not(feature = "local-env"))]
pub mod graceful_shutdown_prod;

#[cfg(feature = "local-env")]
pub use graceful_shutdown_dev::wait_for_shutdown;

#[cfg(not(feature = "local-env"))]
pub use graceful_shutdown_prod::wait_for_shutdown;