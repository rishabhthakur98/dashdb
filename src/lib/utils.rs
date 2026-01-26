#[macro_export]
macro_rules! printerror {
    ($op:expr) => {
        match $op {
            Ok(val) => val,
            Err(e) => {
                eprintln!("Error! {:?}", e);
                return false;
            }
        }
    };
}
