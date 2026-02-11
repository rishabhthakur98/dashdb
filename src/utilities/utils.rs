#[macro_export]
macro_rules! printerror {
    ($op:expr) => {
        match $op.await {
            Ok(val) => val,
            Err(e) => {
                eprintln!("Operation failed {:?}", e);
                return false;
            }
        }
    };
}
