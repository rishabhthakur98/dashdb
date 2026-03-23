#[cfg(feature = "local-env")]
pub mod envvariablesdev;

#[cfg(not(feature = "local-env"))]
pub mod envvariablesprod;

#[cfg(feature = "local-env")]
pub use envvariablesdev::load_and_log_env;

#[cfg(not(feature = "local-env"))]
pub use envvariablesprod::load_and_log_env;

pub struct EnvVariables {
    pub server_url: String,
    pub auth_token: String
}