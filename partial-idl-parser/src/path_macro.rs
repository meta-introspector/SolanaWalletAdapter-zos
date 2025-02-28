/// Get the IDL from the default directory `../../target/idl/` given a program name
#[macro_export]
macro_rules! idl_path {
    ($program_name:expr) => {
        include_str!(concat!("../../target/idl/", $program_name, ".json"))
    };
}

/// Get the IDL from the custom directory given a program name.
#[macro_export]
macro_rules! idl_custom_path {
    ($relative_dir_path:expr, $program_name:expr) => {
        include_str!(concat!($relative_dir_path, $program_name, ".json"))
    };
}
