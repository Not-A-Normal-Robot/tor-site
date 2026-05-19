use std::ffi::OsString;

pub static VAR_NAME_UDS_PATH: &str = "UDS_PATH";

pub fn get_uds_path() -> Option<OsString> {
    std::env::var_os(VAR_NAME_UDS_PATH)
}
