#[macro_export]
macro_rules! printerror {
    ($op:expr) => {
        match $op {
            Ok(val) => val,
            Err(e) => {
                tracing::error!(error = %e, "Operation failed");
                return false;
            }
        }
    };
}
